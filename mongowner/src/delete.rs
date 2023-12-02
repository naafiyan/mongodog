use crate::util::*;

use async_recursion::async_recursion;
use futures::future::try_join_all;
use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::{bson::doc, Database};
use petgraph::{graphmap::GraphMap, Directed, Direction};
use std::fmt::Debug;

/// The `Schemable` trait provides the details associated with a data model struct,
/// necessary to safely delete it and all the data an instance of this model owns.
pub trait Schemable {
    type Value: Debug;
    fn struct_name() -> &'static str;
    fn collection_name() -> &'static str;
    fn cascade_delete(&self);
    fn index_name() -> &'static str;
    fn index_value(&self) -> Self::Value;
}

/// Safe deletion for an object that implements the `Schemable` trait, where "safety"
/// is defined as the property that deleting a `Schemable` deletes all of the data it
/// exclusively owns, i.e. leaves no orphaned data.
pub async fn safe_delete<T: Schemable>(
    to_delete: T,
    db: &Database,
) -> Result<(), Box<dyn std::error::Error>>
where
    mongodb::bson::Bson: From<<T as Schemable>::Value>,
{
    let mut contents = String::new();
    let graph = load_graph(&mut contents)?;

    // Look up to_delete.collection_name in the graph to get a starting point
    // User
    let curr_coll_name = T::collection_name();

    // Get the immediate neighboring edges of to_delete to structs that to_delete owns
    let edges_to_children = graph.edges_directed(&curr_coll_name, Direction::Incoming);

    for (child_coll, _, edge) in edges_to_children {
        let collection = db.collection::<Document>(child_coll);
        let found_cursor = collection
            .find(doc! { edge.owned_field: to_delete.index_value() }, None)
            .await?;

        // Call safe_delete_document on each of the found results. Join all of the resulting promises.
        let doc_vec: Vec<Document> = found_cursor.try_collect().await?;
        let delete_promises: Vec<_> = doc_vec
            .iter()
            .map(|doc| safe_delete_children(doc, child_coll, &graph, &db))
            .collect();
        try_join_all(delete_promises).await?;

        // Call delete_many on all found elements of our collection
        collection
            .delete_many(doc! { edge.owned_field: to_delete.index_value() }, None)
            .await?;
    }
    // Delete to_delete
    db.collection::<T>(curr_coll_name)
        .delete_one(doc! { T::index_name() : to_delete.index_value() }, None)
        .await?;

    Ok(())
}

/// Helper function for safe_delete that operates on documents instead of schemables.
/// This is the function that recurs internally when a user calls safe_delete. This does
/// not delete the inputted document itself, but it does delete everything that document
/// directly or indirectly owns.
/// Note: the ?Send annotation prevents communication between threads; this is a quick
/// fix to the dyn Error type being un-Send-able. May revisit.
#[async_recursion(?Send)]
async fn safe_delete_children<'a>(
    to_delete: &Document,
    collection_name: &str,
    graph: &GraphMap<&str, OwnEdge<'a>, Directed>,
    db: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get edges incoming towards to_delete's collection
    let edges_to_children = graph.edges_directed(&collection_name, Direction::Incoming);
    let mut owner_id = None;

    // Recursively call safe_delete_document on every document that the current
    // document owns
    for (child_coll, _, edge) in edges_to_children {
        println!("Edge: {:#?}", edge);
        let collection = db.collection::<Document>(child_coll);
        // Get the owner's id if we haven't already
        if owner_id.is_none() {
            owner_id = to_delete.get(edge.owner_index).unwrap().into();
        }
        let found_cursor = collection
            .find(doc! { edge.owned_field: owner_id.unwrap() }, None)
            .await?;
        // Call safe_delete_document on each of the found results. Join all of the resulting promises.
        let doc_vec: Vec<Document> = found_cursor.try_collect().await?;
        let delete_promises: Vec<_> = doc_vec
            .iter()
            .map(|doc| safe_delete_children(doc, child_coll, &graph, &db))
            .collect();
        try_join_all(delete_promises).await?;
        // Call delete_many on all found elements of our collection
        collection
            .delete_many(doc! { edge.owned_field: owner_id.unwrap() }, None)
            .await?;
    }

    Ok(())
}

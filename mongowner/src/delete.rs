use mongodb::bson::Document;
use mongodb::{bson::doc, bson::uuid::Uuid, Collection, Database};
use petgraph::{graphmap, Directed, Direction};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs, io::Read};

/// The `Schemable` trait provides the details associated with a data model struct,
/// necessary to safely delete it and all the data an instance of this model owns.
pub trait Schemable {
    fn struct_name() -> &'static str;
    fn collection_name() -> &'static str;
    fn cascade_delete(&self);
    fn index_name() -> &'static str;
    // TODO: N - make this a generic return type so it supports any type that users have
    fn index_value(&self) -> Uuid;
}

/// Represents an edge between two structs.
/// Ex. for User, Post, we would have owner_index = user_id, owned_field = posted_by
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct OwnEdge<'a> {
    owner_index: &'a str,
    owned_field: &'a str,
}

/// Safe deletion for an object that implements the `Schemable` trait, where "safety"
/// is defined as the property that deleting a `Schemable` deletes all of the data it
/// exclusively owns, i.e. leaves no orphaned data.
pub async fn safe_delete<T: Schemable>(
    to_delete: T,
    db: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("DEBUG: entered safe_delete");
    // Reference the graph in env::var("OUT_DIR")
    // TODO: move graph-reading code out into a util function
    let out_dir = env::var("OUT_DIR").unwrap();
    let dir_path = Path::new(&out_dir);
    let graph_path = dir_path.join("graph.json");
    let mut file = fs::File::open(graph_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let graph: graphmap::GraphMap<&str, OwnEdge, Directed> = match serde_json::from_str(&contents) {
        Ok(g) => g,
        Err(_) => graphmap::GraphMap::new(),
    };

    // Look up to_delete.collection_name in the graph to get a starting point
    // User
    let curr_coll_name = T::collection_name();

    // Get the immediate neighboring edges of to_delete to structs that to_delete owns
    let edges_to_children = graph.edges_directed(&curr_coll_name, Direction::Incoming);

    for (child_coll, _, edge) in edges_to_children {
        println!("DEBUG: iter {:?}, edge {:?}", &child_coll, &edge);
        let collection = db.collection::<Document>(child_coll);
        let mut found_cursor = collection
            .find(doc! {edge.owned_field : to_delete.index_value()}, None)
            .await?;
        while found_cursor.advance().await? {
            let curr_child = found_cursor.deserialize_current()?;
            println!("DEBUG: curr_child: {:?}", &curr_child);
            // safe_delete(curr_child, db).await?;
        }
    }
    // Delete to_delete
    // db.collection(curr_struct_name).delete_one(doc! {to_delete: }, options)
    Ok(())
}

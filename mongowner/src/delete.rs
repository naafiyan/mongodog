use mongodb::bson::Document;
use mongodb::{bson::doc, bson::uuid::Uuid, Collection, Database};
use petgraph::{graphmap, Directed, Direction};
use std::path::Path;
use std::{env, fs, io::Read};

/// The `Schemable` trait provides the details associated with a data model struct,
/// necessary to safely delete it and all the data an instance of this model owns.
pub trait Schemable {
    fn collection_name(&self) -> &'static str;
    fn cascade_delete(&self);
    fn index_name(&self) -> &'static str;
    fn index_value(&self) -> Uuid;
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
    let graph: graphmap::GraphMap<&str, &str, Directed> = match serde_json::from_str(&contents) {
        Ok(g) => g,
        Err(_) => graphmap::GraphMap::new(),
    };

    // Look up to_delete.collection_name in the graph to get a starting point
    // User
    let curr_struct_name = to_delete.collection_name();

    // Get the immediate neighboring edges of to_delete to structs that to_delete owns
    let edges_to_children = graph.edges_directed(curr_struct_name, Direction::Incoming);

    let collection = db.collection::<Document>("posts");
    let one_doc = collection.find(doc! {}, None).await?;
    let one_doc_deser = one_doc.deserialize_current()?;
    println!(
        "DEBUG: post is posted by: {:?}",
        one_doc_deser.get("posted_by")
    );

    // for (child_struct, _, index_field) in edges_to_children {
    // let collection = db.collection::<Document>(child_struct);
    // let mut found_cursor = collection
    //     .find(doc! {*index_field: to_delete.index_value()}, None)
    //     .await?;
    // while found_cursor.advance().await? {
    //     let curr_child = found_cursor.deserialize_current()?;
    //     safe_delete(curr_child, db).await?;
    // }

    // }

    // Delete to_delete
    // db.collection(curr_struct_name).delete_one(doc! {to_delete: }, options)

    println!("hello world from generated safe_delete :)");
    Ok(())
}

// use mongodb :: bson :: uuid :: Uuid ; use mongodb :: Database ; # [doc = " The Schemable trait provides the details associated with a data model struct"] # [doc = " necessary to safely delete it and all the data an instance of this model owns."] pub trait Schemable { fn collection_name () -> & 'static str ; fn cascade_delete (& self) ; fn index_name () -> & 'static str ; fn index_value (& self) -> Uuid ; } # [doc = " Safe deletion for an object that implements the `Schemable` trait, where \"safety\""] # [doc = " is defined as the property that deleting a Schemable deletes all of the data it"] # [doc = " exclusively owns, i.e. leaves no orphaned data."] pub fn safe_delete < T : Schemable > (to_delete : & T , db : & Database) { println ! ("hello world from generated safe_delete :)") ; }

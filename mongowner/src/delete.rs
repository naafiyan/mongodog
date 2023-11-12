use mongodb::{bson::doc, bson::uuid::Uuid, Collection, Database};
use petgraph::{graphmap, Directed};
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

    // Get the immediate neighbors of to_delete, i.e. structs that to_delete owns
    // Post, Comment
    let neighbor_structs =
        graph.neighbors_directed(curr_struct_name, petgraph::Direction::Incoming);

    for neighbor in neighbor_structs {
        let collection = db.collection::<Box<dyn Schemable>>(&neighbor);
        // let found_neighbors = collection.find(doc! { }, options)
    }

    // For each of these structs s:
    // for neighbor in neighbor_structs {
    //     // Lookup and delete all elements x of s.collection_name such that to_delete.index_name == x.index_value.

    //     // NOTE: we need to get know the type of the collection that we are parsing for our lookup.
    //     //   For this purpose, we've used a Box of a dynamic trait.
    //     db.collection::<Box<dyn Schemable>>(&neighbor)
    //         .delete_many(
    //             doc! {to_delete.index_name() : to_delete.index_value()},
    //             None,
    //         )
    //         .await?;
    // }

    // Delete to_delete
    // db.collection(curr_struct_name).delete_one(doc! {to_delete: }, options)

    println!("hello world from generated safe_delete :)");
    Ok(())
}

// use mongodb :: bson :: uuid :: Uuid ; use mongodb :: Database ; # [doc = " The Schemable trait provides the details associated with a data model struct"] # [doc = " necessary to safely delete it and all the data an instance of this model owns."] pub trait Schemable { fn collection_name () -> & 'static str ; fn cascade_delete (& self) ; fn index_name () -> & 'static str ; fn index_value (& self) -> Uuid ; } # [doc = " Safe deletion for an object that implements the `Schemable` trait, where \"safety\""] # [doc = " is defined as the property that deleting a Schemable deletes all of the data it"] # [doc = " exclusively owns, i.e. leaves no orphaned data."] pub fn safe_delete < T : Schemable > (to_delete : & T , db : & Database) { println ! ("hello world from generated safe_delete :)") ; }

use dotenv::dotenv;
use petgraph::{graphmap::GraphMap, Directed};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fmt::Debug, fs, io::Read};

/// Represents an edge between two structs.
/// Ex. for User, Post, we would have owner_index = user_id, owned_field = posted_by
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OwnEdge<'a> {
    pub owner_index: &'a str,
    pub owned_field: &'a str,
}

/// Accepts a mutable string buffer and returns the graph stored in the path {OUT_DIR}/graph.json.
pub fn load_graph<'a>(
    contents: &'a mut String,
) -> Result<GraphMap<&'_ str, OwnEdge<'_>, Directed>, Box<dyn std::error::Error>> {
    dotenv().ok();
    // Reference the graph in env::var("CARGO_MANIFEST_DIR")
    let out_dir =
        env::var("CARGO_MANIFEST_DIR").expect("Error reading CARGO_MANIFEST_DIR env variable");
    let dir_path = Path::new(&out_dir);
    let graph_path = dir_path
        .join("target")
        .join(std::env::var("GRAPH_NAME").unwrap_or("graph.json".to_string()));
    let mut file = fs::File::open(graph_path)?;
    file.read_to_string(contents)?;
    let graph: GraphMap<&str, OwnEdge, Directed> = match serde_json::from_str(contents) {
        Ok(g) => g,
        Err(_) => GraphMap::new(),
    };
    println!("{:#?}", graph);
    Ok(graph)
}

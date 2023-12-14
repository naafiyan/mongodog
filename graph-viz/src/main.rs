use std::env;
use std::fmt::{self, Display};
use std::fs::OpenOptions;
use std::{io::Read, path::Path};

use petgraph::dot::{Config, Dot};
use petgraph::prelude::GraphMap;
use serde::{Deserialize, Serialize};

/// Represents an edge between two structs.
/// Ex. for User, Post, we would have owner_index = user_id, owned_field = posted_by
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OwnEdge<'a> {
    pub owner_index: &'a str,
    pub owned_field: &'a str,
}

impl<'a> fmt::Display for OwnEdge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Owner Index: '{}', Owned Field: '{}'",
            self.owner_index, self.owned_field
        )
    }
}
fn main() {
    // load the json graph
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let graph_path = Path::new(file_path);
    let mut file = std::fs::File::open(graph_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let graph: GraphMap<&str, OwnEdge, petgraph::Directed> = match serde_json::from_str(&contents) {
        Ok(g) => g,
        Err(_) => GraphMap::new(),
    };
    // write the Dot graph to a file
    let dot_graph = Dot::new(&graph);
    println!("{:?}", dot_graph);
    // TODO: make it a dot file for graph viz
}

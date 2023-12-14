use std::fmt::{self, Display};
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
    let graph_path = Path::new("graph.json");
    let mut file = std::fs::File::open(graph_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let graph: GraphMap<&str, OwnEdge, petgraph::Directed> = match serde_json::from_str(&contents) {
        Ok(g) => g,
        Err(_) => GraphMap::new(),
    };
    println!("{:?}", Dot::new(&graph));

    // TODO: make it a dot file for graph viz
}

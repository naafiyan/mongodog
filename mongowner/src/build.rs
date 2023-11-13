extern crate proc_macro;

use quote::quote;
use syn::Item;
use syn::ItemFn;
use std::fs;
use syn::parse_str;
use syn::File;
use std::path::Path;
// import serde_json and petgraph
use serde_json;
use petgraph::{algo::is_cyclic_directed, graphmap, Directed};


fn main() {
    // either opens or creates a delete_original.rs file if it doesn't already exist
    // all future compilations will derive from the delete_original.rs since AST parsing will be
    // standard
    let source = match fs::read_to_string("src/delete_original.rs") {
        Ok(file_path) => file_path,
        Err(_err) => {
            fs::File::create("src/delete_original.rs").expect("Failed to create delete_original.rs");
            fs::copy("src/delete.rs", "src/delete_original.rs").expect("Failed to setup delete.rs files");
            fs::read_to_string("src/delete_original.rs").expect("Failed to open delete_original.rs")
        }
    };

    // TODO: ensure that this new script is run EVERYTIME a Schema object changes

    let path = Path::new("social-rs/server/data/graph.json");
    let display = path.display();
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    let mut graph: graphmap::GraphMap<&str, &str, Directed> = match serde_json::from_str(&contents)
    {
        Ok(g) => g,
        Err(_) => graphmap::GraphMap::new(),
    };
    let graph = graph.into_graph::<u32>();
    println!("DEBUG: ownership graph: {:?}", &graph);
    println!(
        "VALIDATION: graph is not cyclic: {:?}",
        !is_cyclic_directed(&graph)
    );


    // parse the delete_origina.rs into an AST
    let mut ast = parse_str::<File>(&source).unwrap();

    // body of the safe_delete function
    let new_body = quote! {
        {
            println!("hello world from generated safe_delete 2 :)");
        }
    };

    // go in and modify the AST representation of the safe_delete function to overwrite the body
    for item in &mut ast.items {
        if let Item::Fn(item_fn) = item {
            if item_fn.sig.ident.to_string() == "safe_delete" {
                let mut cloned_item_fn = item_fn.clone();
                cloned_item_fn.block = parse_str(&new_body.to_string()).expect("Failed to parse new body");
                *item_fn = ItemFn { attrs: cloned_item_fn.attrs, vis: cloned_item_fn.vis, sig: cloned_item_fn.sig, block: cloned_item_fn.block }
            }
        }
    }

    // actually write the bytes to delete.rs
    let modified_source = format!("{}", quote! { #ast });
    fs::write("src/delete.rs", modified_source).expect("Failed to write to delete.rs");
}

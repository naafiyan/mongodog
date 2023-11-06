extern crate proc_macro;

use quote::quote;
use syn::Expr;
use syn::Ident;
use syn::Item;
use syn::ItemFn;
use syn::Token;
use syn::parse::Parser;
use syn::token::Fn;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use syn::parse_file;
use syn::parse_str;
use syn::parse2;
use proc_macro::TokenStream;
use syn::File;

// example of a main that does basic File I/O
// fn main() {
//     // TODO: ensure that this script is run EVERYTIME a Schema object changes
//     let path = Path::new("src/hello_world.txt");
//     let display = path.display();
//     // Open the path in read-only mode, returns `io::Result<File>`
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Read the file contents into a string, returns `io::Result<usize>`
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display, why),
//         Ok(_) => print!("{} contains:\n{}", display, s),
//     }
// }
//

fn main() {
    // open original template delete.rs
    let source = fs::read_to_string("src/delete_original.rs").expect("Failed to read delete.rs");
    let mut ast = parse_str::<File>(&source).unwrap();

    let new_body = quote! {
        {
            println!("hello world from generated safe_delete :)");
        }
    };

    for item in &mut ast.items {
        if let Item::Fn(item_fn) = item {
            if item_fn.sig.ident.to_string() == "safe_delete" {
                println!("Found safe_delete function:\n{:#?}", item_fn);
                let mut cloned_item_fn = item_fn.clone();
                cloned_item_fn.block = parse_str(&new_body.to_string()).expect("Failed to parse new body");
                *item_fn = ItemFn { attrs: cloned_item_fn.attrs, vis: cloned_item_fn.vis, sig: cloned_item_fn.sig, block: cloned_item_fn.block }
            }
        }
    }

    let modified_source = format!("{}", quote! { #ast });
    fs::write("src/delete.rs", modified_source).expect("Failed to write to delete.rs");
}

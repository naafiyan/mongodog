extern crate proc_macro;

use quote::quote;
use syn::Item;
use syn::ItemFn;
use std::fs;
use syn::parse_str;
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

    // parse the delete_origina.rs into an AST
    let mut ast = parse_str::<File>(&source).unwrap();

    // body of the safe_delete function
    let new_body = quote! {
        {
            println!("hello world from generated safe_delete :)");
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

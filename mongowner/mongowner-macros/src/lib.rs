extern crate proc_macro;
use dotenv::dotenv;
use petgraph::{algo::is_cyclic_directed, graphmap, Directed};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    env,
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Meta};

// enum to represent all the types of schema annotations
enum SchemaAnnotations {
    OwnedBy,
    Index,
    CollectionName,
    DataSubject,
}

/// Represents an edge between two structs.
/// Ex. for User, Post, we would have owner_index = user_id, owned_field = posted_by
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct OwnEdge<'a> {
    owner_index: &'a str,
    owned_field: &'a str,
}

impl SchemaAnnotations {
    fn as_str(&self) -> &'static str {
        match self {
            SchemaAnnotations::Index => "index",
            SchemaAnnotations::OwnedBy => "owned_by",
            SchemaAnnotations::CollectionName => "collection",
            SchemaAnnotations::DataSubject => "data_subject",
        }
    }
}

/// A custom derive macro meant for data model structs that are connected, in some way,
/// to a data subject. This produces an implementation of the `Schemable` trait
/// for this struct.
/// - The #[collection(_)] macro helps identify the name of the collection associated
/// with the struct in Mongo.
/// - The #[owned_by(_)] macro is used to annotate fields containing references to other
/// models or collections.
/// - The #[index] macro is used to annotate fields that are primary key of the model
/// - The #[data_subject] macro is used to annotate structs that are data subjects
#[proc_macro_derive(Schema, attributes(owned_by, collection, index, data_subject))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    dotenv().ok();
    println!("Proc macroing it up");
    // Parse the collection name from the #[collection(_)] annotation.
    let input = parse_macro_input!(input as DeriveInput);

    let collection_name =
        match parse_header_annotation(&input, SchemaAnnotations::CollectionName.as_str()) {
            Some(name) => name,
            None => panic!("All schemas must have collection names"),
        };

    // TODO: N - this might not actually be needed at all
    // whether or not the given input model is a data_subject
    let is_data_subj =
        match parse_header_annotation(&input, SchemaAnnotations::DataSubject.as_str()) {
            Some(_) => true,
            _ => false,
        };

    // Identify the Rust struct associated with the input string (eg. "User" -> User)
    let curr_struct_type = generate_struct_type(input.ident.to_string());

    let fields = extract_fields_from_schema(input);
    let owned_by_fields = find_fields_by_annotation(&fields, SchemaAnnotations::OwnedBy.as_str());

    if is_data_subj {
        if let Some(_) = owned_by_fields {
            panic!("Data subject cannot have any owned_by field");
        }
    } else {
        if let None = owned_by_fields {
            panic!("Non data subjects MUST have atleast ONE owned_by field");
        }
    }

    let index_field = match find_field_by_annotation(&fields, SchemaAnnotations::Index.as_str()) {
        Some(res) => res,
        None => panic!("Error finding index_field"),
    };

    let index_field_name = find_field_name(index_field);
    if let Err(e) = add_index_to_file(&collection_name, &index_field_name) {
        panic!("Error {:#?} adding index to index file", e);
    }

    let index_ident = Ident::new(&index_field_name, proc_macro2::Span::call_site());
    // let index_type = find_field_type(index_field);
    let index_type_ident = { index_field.ty.clone() };

    let curr_node_name = curr_struct_type.to_string();

    if let Some(fields) = owned_by_fields {
        for field in fields {
            let reference_field = field.ident.as_ref().unwrap().to_string();
            // TODO: refactor
            let _ = &field
                .attrs
                .get(0)
                .expect("Error getting field.attrs.get(0)")
                .parse_nested_meta(|meta| {
                    let attr_input_stream = meta.input.cursor().token_stream();
                    let edge_field_opt = attr_input_stream
                        .into_iter()
                        .skip(1)
                        .next()
                        .and_then(|t| Some(t.to_string()));
                    if edge_field_opt == None {
                        return Ok(());
                    }
                    let edge_field_name = edge_field_opt.expect("Error getting edge field_name");

                    let owner_coll_name = meta
                        .path
                        .get_ident()
                        .unwrap_or_else(|| panic!("no owner argument in owned_by annotation"))
                        .to_string();

                    let edge = OwnEdge {
                        owner_index: &edge_field_name,
                        owned_field: &reference_field,
                    };

                    // graph defaults to target dir
                    let out_dir = env::var("CARGO_MANIFEST_DIR")
                        .expect("Error reading CARGO_MANIFEST_DIR env variable");
                    let dir_path = Path::new(&out_dir);

                    let graph_path = dir_path
                        .join("target")
                        .join(std::env::var("GRAPH_NAME").unwrap_or("graph.json".to_string()));
                    match add_edge_to_file(&collection_name, &owner_coll_name, edge, &graph_path) {
                        Err(e) => panic!("Could not add edge to file: {:?}", e),
                        Ok(_) => (),
                    };

                    Ok(())
                });
        }
    }

    // TODO: actually generate the index on the given field and collection
    // TODO: handle non Uuid field type values
    // TODO: currently having to hardcode Uuid import and type rather than generically determining
    // it

    let gen = quote! {
        impl Schemable for #curr_struct_type {
            type Value = #index_type_ident;
            fn struct_name() -> &'static str {
                #curr_node_name
            }
            fn collection_name() -> &'static str {
                #collection_name
            }
            fn cascade_delete(&self) {
                // delete all documents in ALL fk (owned) schemas where value(fk_field_name) = self._id
                // delete self from self.collection
                // TODO: have to have some way of getting and storing the collection name of
                // both the owner and owned_by schemas
            }
            fn index_name() -> &'static str {
                #index_field_name
            }
            fn index_value(&self) -> Self::Value {
                self.#index_ident.clone()
            }
        };
    };
    return gen.into();
}

fn add_index_to_file(
    collection_name: &str,
    index_field_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir =
        env::var("CARGO_MANIFEST_DIR").expect("Error reading CARGO_MANIFEST_DIR env variable");
    let dir_path = Path::new(&out_dir);
    let map_path = dir_path
        .join("target")
        .join(std::env::var("INDEX_NAME").unwrap_or("index_map.json".to_string()));
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&map_path)?;

    let saved_position = file.seek(SeekFrom::Current(0))?;

    // read file contents to obtain existing map
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut map: HashMap<&str, &str> = match serde_json::from_str(&contents) {
        Ok(g) => g,
        Err(_) => HashMap::new(),
    };

    match map.insert(collection_name, index_field_name) {
        Some(s) => println!("Detected collision in index map with value {:#?}", s),
        None => println!(
            "New index map entry with {:#?} : {:#?}",
            collection_name, index_field_name
        ),
    };

    file.seek(SeekFrom::Start(saved_position))?;

    // write the modified graph back into the file
    let serialized_map = serde_json::to_string(&map).unwrap();
    file.write_all(serialized_map.as_bytes())?;

    Ok(())
}

/// Reads the file containing the serialized graph (or creates this file if it doesn't exist),
/// and writes a modified graph to the file that also contains an edge between `a` and `b`.
fn add_edge_to_file(
    owned_node: &str,
    owner_node: &str,
    edge: OwnEdge,
    filepath: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // open the file in question and create it if it doesn't exist
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&filepath)?;

    // save starting position of file to seek
    let saved_position = file.seek(SeekFrom::Current(0))?;

    // read file contents to obtain existing graph
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut graph: graphmap::GraphMap<&str, OwnEdge, Directed> =
        match serde_json::from_str(&contents) {
            Ok(g) => g,
            Err(_) => graphmap::GraphMap::new(),
        };

    // add the edge to the graph
    let node_a = if graph.contains_node(owned_node) {
        owned_node
    } else {
        graph.add_node(owned_node)
    };
    let node_b = if graph.contains_node(owner_node) {
        owner_node
    } else {
        graph.add_node(owner_node)
    };
    graph.add_edge(node_a, node_b, edge);

    // Check that adding this edge doesn't introduce a cycle
    if is_cyclic_directed(&graph) {
        println!("\nDEBUG: cycle detected\n");
        return Err(format!(
            "Adding edge ({}, {}) created cycle in ownership graph.",
            node_a, node_b
        )
        .into());
    }

    // restore the saved position
    file.seek(SeekFrom::Start(saved_position))?;

    // write the modified graph back into the file
    let serialized_graph = serde_json::to_string(&graph).unwrap();
    file.write_all(serialized_graph.as_bytes())?;

    Ok(())
}

// generates a Ident that can be used with the # operator in quote to reference actual Rust defined
// types
fn generate_struct_type(type_str: String) -> Ident {
    syn::Ident::new(&type_str, proc_macro2::Span::call_site())
}

// parse the header annotations of a struct
fn parse_header_annotation(input: &DeriveInput, annotation: &str) -> Option<String> {
    let mut a = None;
    for attr in &input.attrs {
        if let Meta::Path(mp) = &attr.meta {
            for seg in &mp.segments {
                if seg.ident.to_string() == annotation {
                    a = Some(seg.ident.to_string());
                    break;
                }
            }
        }
        if let Meta::List(ml) = &attr.meta {
            for seg in &ml.path.segments {
                if seg.ident.to_string() == annotation {
                    a = Some(ml.tokens.to_string());
                    break;
                }
            }
        }
    }
    a
}

// fn find_field_type(field: &Field) -> String {
//     // TODO: a few cases to handle -
//     // type name is just one path length e.g. Uuid
//     // path length > 1 e.g. mongowner::mongo::bson::uuid::Uuid
//     match &field.ty {
//         syn::Type::Path(p) => p
//             .path
//             .segments
//             .iter()
//             .map(|s| s.ident.to_string())
//             .collect::<Vec<String>>()
//             .join("::"),
//         // syn::Type::Path(p) => match p.path.segments.first() {
//         //     Some(ps) => ps.ident.to_string(),
//         //     None => panic!("No type found for field"),
//         // },
//         _ => panic!("error parsing field!"),
//     }
// }
//
// extract the fields from a given schema
fn extract_fields_from_schema(input: DeriveInput) -> FieldsNamed {
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields
        } else {
            panic!("Could not read any fields from struct")
        }
    } else {
        panic!("Could not read any fields from struct")
    }
}

// generic field parser to find annotated fields
fn find_field_by_annotation<'a>(fields: &'a FieldsNamed, annotation: &str) -> Option<&'a Field> {
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                if let Meta::Path(ml) = &attr.meta {
                    for seg in &ml.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            return Some(field);
                        }
                    }
                }
                if let Meta::List(ml) = &attr.meta {
                    for seg in &ml.path.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            return Some(field);
                        }
                    }
                }
            }
        }
    }
    None
}

// given a syn::Field object, it returns the string name of the annotated field
fn find_field_name(field: &Field) -> String {
    match &field.ident {
        Some(i) => i.to_string(),
        None => panic!("Could not find annotated field"),
    }
}

// generic field parser to find annotated fields
fn find_fields_by_annotation<'a>(
    fields: &'a FieldsNamed,
    annotation: &str,
) -> Option<Vec<&'a Field>> {
    let mut found: Vec<&Field> = Vec::new();
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                if let Meta::Path(ml) = &attr.meta {
                    for seg in &ml.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            found.push(field);
                        }
                    }
                }
                if let Meta::List(ml) = &attr.meta {
                    for seg in &ml.path.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            found.push(field);
                        }
                    }
                }
            }
        }
    }
    if found.len() > 0 {
        return Some(found);
    }
    None
}

// test will be
// test safe_delete(DATA_SUBJECT) e.g. User
// safe_delete(User) -> extract User._id -> recurse on all things that are owned_by User given the
// user_id
// safe_delete(ds: Data_Subject) {
//  get ds._id
// }
//
// if post has user as owner then user being deleted means delete post
//
//
// TODO: derive macro that implements functions for deletion and access
// TODO: think about how to link user struct and post struct
// TODO: start with issuing deletion function on a single struct
// OR
// info being passed to the macro gets stored in a data structure somewhere
//
//
// Application calls delete on User struct (gives it user key) -> macro figures out what else
// depends on the user struct and recurses and so would have to provide the foreign key
//
// macro takes in additional parameters that handles which fields refer to which collection
// simplify to OWNED_BY for now and then OWNS as a long time goal
// think about shared data

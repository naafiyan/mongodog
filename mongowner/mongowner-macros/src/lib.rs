extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Meta};

// enum to represent all the types of schema annotations
enum SchemaAnnotations {
    OwnedBy,
    Index,
    CollectionName
}

impl SchemaAnnotations {
    fn as_str(&self) -> &'static str {
        match self {
            SchemaAnnotations::Index => "index",
            SchemaAnnotations::OwnedBy => "owned_by",
            SchemaAnnotations::CollectionName => "collection"
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
#[proc_macro_derive(Schema, attributes(owned_by, collection, index))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    // Parse the collection name from the #[collection(_)] annotation.
    let input = parse_macro_input!(input as DeriveInput);
    println!("input: {:?}", input);

    let collection_name = match parse_collection_name(&input) {
        Some(name) => name,
        None => panic!("All schemas must have collection names")
    };

    // Identify the Rust struct associated with the input string (eg. "User" -> User)
    let curr_struct_type = generate_struct_type(input.ident.to_string());

    let fields = extract_fields_from_schema(input);
    let owned_by_field = find_field_by_annotation(&fields, SchemaAnnotations::OwnedBy.as_str());
    let index_field = find_field_by_annotation(&fields, SchemaAnnotations::Index.as_str());

    let owned_by_field_name = find_field_name(owned_by_field);
    let index_field_name = find_field_name(index_field);

    println!("owned_by_field_name: {:?}", owned_by_field_name);
    println!("index_field: {:?}", index_field_name);
    

    // TODO: handle the case where there is NO owned_by annotation, i.e. data subject
    // curent approach of just unwrapping causes a panic since we might be unwrapping a None
    // object when there is no owned_by annotation
    //
    // let field = find_fk_field(&fields).unwrap();
    // let owner_type_ident = {
    //     let owner_type = find_owner(&field).unwrap();
    //     syn::Ident::new(&owner_type, proc_macro2::Span::call_site())
    // };
    // let fk_field_name = {
    //     match &field.ident {
    //         Some(ident) => ident.to_string(),
    //         None => "".to_string()
    //     }
    // };
    let gen = quote! {
        impl Schemable for #curr_struct_type {
            fn collection_name() -> &'static str {
                #collection_name
            }
            fn cascade_delete(&self) {
                // delete all documents in ALL fk (owned) schemas where value(fk_field_name) = self._id
                // delete self from self.collection
                // TODO: have to have some way of getting and storing the collection name of
                // both the owner and owned_by schemas
            }
        };
    };
    return gen.into();
}

// generates a Ident that can be used with the # operator in quote to reference actual Rust defined
// types
fn generate_struct_type(type_str: String) -> Ident {
    syn::Ident::new(&type_str, proc_macro2::Span::call_site())
}

// Parses the #[collection_name] annotation from the AST of a struct
fn parse_collection_name(input: &DeriveInput) -> Option<String> {
    let mut a = None;
    for attr in &input.attrs {
        if let Meta::List(ml) = &attr.meta {
            for seg in &ml.path.segments {
                if seg.ident.to_string() == SchemaAnnotations::CollectionName.as_str() {
                    a = Some(ml.tokens.to_string());
                    break;
                }
            }
        }
    }
    a
}

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
fn find_field_by_annotation<'a>(fields: &'a FieldsNamed, annotation: &str) -> &'a Field {
    println!("Looking for: {:?}", annotation);
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                println!("attr - {:#?}", attr);
                if let Meta::Path(ml) = &attr.meta {
                    for seg in &ml.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            return field
                        }
                    }
                }
                if let Meta::List(ml) = &attr.meta {
                    println!("ml.path.segments - {:#?}", &ml.path.segments);
                    for seg in &ml.path.segments {
                        println!("seg.ident.to_string() - {:#?}", seg.ident.to_string());
                        println!("annotation - {:#?}", annotation);
                        if seg.ident.to_string() == annotation.to_string() {
                            return field
                        }
                    }
                }
            }
        }
    }
    panic!("Could not find field: {:?}", annotation)
}

// given a syn::Field object, it returns the string name of the annotated field
fn find_field_name(field: &Field) -> String {
    for attr in &field.attrs {
        if let Meta::Path(mp) = &attr.meta {
            for seg in &mp.segments {
                return seg.ident.to_string()
            }
        }
        if let Meta::List(ml) = &attr.meta {
            return ml.tokens.to_string()
        }
    }
    panic!("Error parsing annotated field, {:#?}", field)
}

/// Given a `FieldsNamed` reference to a set of fields, return an option
/// containing the field annotated by #[owned_by(_)]. Return None if
/// no such field is found.
fn find_fk_field(fields: &FieldsNamed) -> Option<&Field> {
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                if let Meta::List(ml) = &attr.meta {
                    for seg in &ml.path.segments {
                        if seg.ident.to_string() == "owned_by".to_string() {
                            return Some(field);
                        }
                    }
                }
            }
        }
    }
    return None;
}

/// Extract the parameter given to a macro of the form #[macro_name(param)] and
/// return it as an option.
fn find_owner(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if let Meta::List(ml) = &attr.meta {
            return Some(ml.tokens.to_string());
        }
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

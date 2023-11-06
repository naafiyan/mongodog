extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Meta};

// enum to represent all the types of schema annotations
enum SchemaAnnotations {
    OwnedBy,
    Index,
    CollectionName,
    DataSubject
}

impl SchemaAnnotations {
    fn as_str(&self) -> &'static str {
        match self {
            SchemaAnnotations::Index => "index",
            SchemaAnnotations::OwnedBy => "owned_by",
            SchemaAnnotations::CollectionName => "collection",
            SchemaAnnotations::DataSubject => "data_subject"
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
    // Parse the collection name from the #[collection(_)] annotation.
    let input = parse_macro_input!(input as DeriveInput);

    let collection_name = match parse_header_annotation(&input, SchemaAnnotations::CollectionName.as_str()) {
        Some(name) => name,
        None => panic!("All schemas must have collection names")
    };

    // whether or not the given input model is a data_subject
    let is_data_subj = match parse_header_annotation(&input, SchemaAnnotations::DataSubject.as_str()) {
        Some(_) => true,
        _ => false
    };

    // Identify the Rust struct associated with the input string (eg. "User" -> User)
    let curr_struct_type = generate_struct_type(input.ident.to_string());

    let fields = extract_fields_from_schema(input);
    let owned_by_field = find_field_by_annotation(&fields, SchemaAnnotations::OwnedBy.as_str());
    let index_field = find_field_by_annotation(&fields, SchemaAnnotations::Index.as_str());

    // the name of the field annotated by #[owned_by]
    let owned_by_field_name = find_field_name(owned_by_field);
    // the name of the field annotated by #[index]
    let index_field_name = find_field_name(index_field);
    // TODO: actually generate the index on the given field and collection
    
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
fn find_field_by_annotation<'a>(fields: &'a FieldsNamed, annotation: &str) -> &'a Field {
    for field in fields.named.iter() {
        if field.attrs.len() > 0 {
            for attr in &field.attrs {
                if let Meta::Path(ml) = &attr.meta {
                    for seg in &ml.segments {
                        if seg.ident.to_string() == annotation.to_string() {
                            return field
                        }
                    }
                }
                if let Meta::List(ml) = &attr.meta {
                    for seg in &ml.path.segments {
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

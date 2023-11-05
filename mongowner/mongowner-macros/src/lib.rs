extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Ident};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Meta};

/// A custom derive macro meant for data model structs that are connected, in some way,
/// to a data subject. This produces an implementation of the `Schemable` trait
/// for this struct.
/// - The #[collection(_)] macro helps identify the name of the collection associated
/// with the struct in Mongo.
/// - The #[owned_by(_)] macro is used to annotate fields containing references to other
/// models or collections.
#[proc_macro_derive(Schema, attributes(owned_by, collection))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    // Parse the collection name from the #[collection(_)] annotation.
    let input = parse_macro_input!(input as DeriveInput);
    // println!("input: {:?}", input);
    // TODO: separate out schema into another macro?
    let collection_name: Option<String> = {
        let mut a = None;
        for attr in &input.attrs {
            if let Meta::List(ml) = &attr.meta {
                for seg in &ml.path.segments {
                    if seg.ident.to_string() == "collection".to_string() {
                        a = Some(ml.tokens.to_string());
                        break;
                    }
                }
            }
        }
        a
    };
    let collection_name = collection_name.unwrap();

    // Identify the Rust struct associated with the input string (eg. "User" -> User)
    let curr_struct = input.ident.to_string();
    let curr_struct_type = syn::Ident::new(&curr_struct, proc_macro2::Span::call_site());

    let fields = extract_fields_from_schema(input);
    if let Some(fields) = fields {
        // TODO: handle the case where there is NO owned_by annotation, i.e. data subject
        // curent approach of just unwrapping causes a panic since we might be unwrapping a None
        // object when there is no owned_by annotation
        println!("curr_struct: {}", curr_struct);
        let mut owner_type_ident: Option<Ident> = None;;
        for field in fields.named.iter() {
            let fk_field_name = {
                match &field.ident {
                    Some(ident) => ident.to_string(),
                    None => "".to_string()
                }
            };
            println!("fk_field_name: {}", fk_field_name);
            let owner_types_str = find_owner(&field).unwrap_or_else(|| {
                "".to_string()
            });
            if(owner_types_str != "") {
                let owner_type_str = owner_types_str.split(',').collect::<Vec<&str>>()[0].to_string();
                owner_type_ident = Some(syn::Ident::new(&owner_type_str, proc_macro2::Span::call_site()));
            }  
        }
        let owner_type = owner_type_ident.unwrap_or_else(
            || panic!("No owner type found for {}", curr_struct)
        );
        print!("owner_type: {} \n", owner_type);
        

        let gen = quote! {
            impl Schemable for #curr_struct_type {
                fn collection_name() -> &'static str {
                    #collection_name
                }
                fn cascade_delete(&self) {
                    
                    println!("safe deleting self");
                    // delete all documents in ALL fk (owned) schemas where value(fk_field_name) = self._id
                    // delete self from self.collection
                    // TODO: have to have some way of getting and storing the collection name of
                    // both the owner and owned_by schemas
                }
            }
        };
        return gen.into();
    }

    TokenStream::new()
}

fn extract_fields_from_schema(input: DeriveInput) -> Option<FieldsNamed> {
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            Some(fields)
        } else {
            // compiler error
            None
        }
    } else {
        None
    }
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

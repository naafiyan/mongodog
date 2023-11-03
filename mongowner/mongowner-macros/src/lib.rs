extern crate proc_macro;
use proc_macro::{TokenStream, Ident};
use quote::quote;
use syn::{FieldsNamed, Attribute, Data, DataStruct, DeriveInput, Field, Fields, ItemStruct, parse_macro_input, Meta, Path, PathSegment};

#[proc_macro_derive(Schema, attributes(owned_by, collection))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // TODO: separate out schema into another macro?
    let collection_name : Option<String> = {
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


    let curr_struct = input.ident.to_string();
    let curr_struct_type = syn::Ident::new(&curr_struct, proc_macro2::Span::call_site());

    let fields = extract_fields_from_schema(input);
    if let Some(fields) = fields {
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
                fn collection_name(&self) -> &'static str {
                    #collection_name
                }
                fn cascade_delete(&self) {
                    // delete all documents in ALL fk (owned) schemas where value(fk_field_name) = self._id
                    // delete self from self.collection
                    // TODO: have to have some way of getting and storing the collection name of
                    // both the owner and owned_by schemas
                }
            }
        };
        return gen.into()
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

fn find_fk_field(fields: &FieldsNamed) -> Option<&Field>{
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

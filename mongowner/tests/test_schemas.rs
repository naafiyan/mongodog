// use std::u32;
//
// use mongodb::{Client, Collection, Database};
// use mongowner::{Schema, Schemable};
// use rand::random;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
//
// /// A set of schemas meant to encompass different ownership relations and structures
//
// #[derive(Schema, Serialize, Deserialize)]
// #[data_subject]
// #[collection(users)]
// struct User {
//     #[index]
//     id: u32,
//     username: String,
//     name: String,
//     email: String,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(posts)]
// struct Post {
//     #[index]
//     id: u32,
//     #[owned_by(users, id)]
//     posted_by: u32,
//     text: String,
//     date: String,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(comments)]
// struct Comment {
//     #[index]
//     id: u32,
//     #[owned_by(users, id)]
//     posted_by: u32,
//     #[owned_by(posts, id)]
//     parent_post: u32,
//     text: String,
//     date: String,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(productives)]
// struct Productive {
//     #[index]
//     id: u32,
//     rated_by: u32,
//     #[owned_by(comments, id)]
//     comment: u32,
//     is_productive: bool,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(attachments)]
// struct Attachment {
//     #[owned_by(users, id)]
//     user: u32,
//     #[owned_by(posts, id)]
//     post: u32,
//     #[owned_by(comments, id)]
//     comment: u32,
//     #[index]
//     resource_id: u32,
//     date: String,
//     moderated_by: u32,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[data_subject]
// #[collection(mediamods)]
// struct MediaMod {
//     #[index]
//     id: u32,
//     media_group: String,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(modposts)]
// struct ModPost {
//     #[index]
//     id: u32,
//     #[owned_by(mediamods, id)]
//     moderator: u32,
//     text: String,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(modresources)]
// struct ModResource {
//     #[index]
//     id: u32,
//     #[owned_by(mediamods, id)]
//     moderator: u32,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(mresource1s)]
// struct MResource1 {
//     #[index]
//     id: u32,
//     r1: String,
//     #[owned_by(modresources, id)]
//     parent_res: u32,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(mresources2)]
// struct MResource2 {
//     #[index]
//     id: u32,
//     r2: String,
//     #[owned_by(modresources, id)]
//     parent_res: u32,
// }
//
// #[derive(Schema, Serialize, Deserialize)]
// #[collection(mresources3)]
// struct MResource3 {
//     #[index]
//     id: u32,
//     r3: String,
//     #[owned_by(modresources, id)]
//     parent_res: u32,
// }

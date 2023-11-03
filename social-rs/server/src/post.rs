use serde::{Deserialize, Serialize};
use crate::user;
use user::User;
use mongowner_macros::Schema;
use mongowner::Schemable;
use mongowner::mongo::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
#[collection(posts)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text: String,
    #[owned_by(User)]
    pub posted_by: u32,
    pub date: String,
}

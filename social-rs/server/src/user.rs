use serde::{Deserialize, Serialize};
use mongowner_macros::Schema;
use mongowner::Schemable;
use mongowner::mongo::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

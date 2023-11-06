use serde::{Deserialize, Serialize};
use mongowner_macros::Schema;
use mongowner::Schemable;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
pub struct User {
    #[index]
    pub user_id: mongowner::mongo::bson::uuid::Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    #[owned_by(User)]
    pub email: String,
}

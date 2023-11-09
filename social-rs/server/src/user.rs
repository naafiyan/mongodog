use mongowner::mongo::bson::uuid::Uuid;
use mongowner::Schemable;
use mongowner_macros::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
#[data_subject]
pub struct User {
    #[index]
    pub user_id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

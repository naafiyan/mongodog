use serde::{Deserialize, Serialize};
use mongowner_macros::Schema;
use mongowner::Schemable;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

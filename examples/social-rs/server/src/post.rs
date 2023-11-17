use mongowner::mongo::bson::uuid::Uuid;
use mongowner::{Schema, Schemable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(posts)]
pub struct Post {
    #[index]
    pub post_id: u32,
    pub text: String,
    #[owned_by(users, user_id)]
    pub posted_by: u32,
    pub date: String,
}

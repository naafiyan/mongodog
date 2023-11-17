use mongowner::mongo::bson::uuid::Uuid;
use mongowner::Schemable;
use mongowner_macros::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(posts)]
pub struct Post {
    #[index]
    pub post_id: Uuid,
    pub text: String,
    #[owned_by(users, user_id)]
    pub posted_by: Uuid,
    pub date: String,
}

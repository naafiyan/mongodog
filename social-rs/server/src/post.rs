use serde::{Deserialize, Serialize};
use mongowner_macros::Schema;
use mongowner::Schemable;

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
#[collection(posts)]
pub struct Post {
    #[index]
    pub post_id: mongowner::mongo::bson::uuid::Uuid,
    pub text: String,
    #[owned_by(User)]
    pub posted_by: mongowner::mongo::bson::uuid::Uuid,
    pub date: String,
}

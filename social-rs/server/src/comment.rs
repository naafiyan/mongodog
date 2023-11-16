use mongowner::Schemable;
use mongowner_macros::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
#[collection(comments)]
pub struct Comment {
    #[index]
    pub comment_id: mongowner::mongo::bson::uuid::Uuid,
    pub text: String,
    #[owned_by(Post, post_id)]
    pub parent_post: mongowner::mongo::bson::uuid::Uuid,
    pub date: String,
}

use serde::{Deserialize, Serialize};
use mongowner_macros::Schema;
use mongowner::Schemable;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
#[collection(posts)]
pub struct Post {
    pub text: String,
    #[owned_by(User, user_id)]
    pub posted_by: Uuid,
    pub date: String,
}

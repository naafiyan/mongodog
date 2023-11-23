use std::process::Command;

use mongodb::{Client, Collection, Database};
use mongowner::{Schema, Schemable};
use rand::random;
use serde::{Deserialize, Serialize};

// Heavily simplified representations of User, Post and Comment for testing
#[derive(Schema, Serialize, Deserialize)]
#[collection(users)]
#[data_subject]
pub struct User {
    #[index]
    pub user_id: i32,
    pub username: String,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(posts)]
pub struct Post {
    #[index]
    pub post_id: i32,
    #[owned_by(users, user_id)]
    pub posted_by: i32,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(comments)]
pub struct Comment {
    #[index]
    pub comment_id: i32,
    #[owned_by(posts, post_id)]
    pub commented_on: i32,
}

pub async fn init_test_db() -> Result<Database, String> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let db_name = format!("test_db{}", random::<u8>());
    let db = client.database(&db_name);
    teardown_db(&db).await;
    Ok(db)
}

pub async fn teardown_db(db: &Database) {
    println!("Dropping: {:#?}", db.name());
    db.drop(None).await.expect("Error dropping db")
}

pub async fn insert_posts(coll: &Collection<Post>, owner_id: i32, count: i32) {
    let mut posts: Vec<Post> = Vec::new();
    for n in 0..count {
        posts.push(Post {
            post_id: n,
            posted_by: owner_id,
        });
    }
    // insert all in one go
    coll.insert_many(posts, None)
        .await
        .expect("Error generating posts");
}

pub async fn insert_user(coll: &Collection<User>, user_id: i32) -> User {
    let user = User {
        user_id,
        username: "Alice".to_string(),
    };
    coll.insert_one(&user, None)
        .await
        .expect("Failed to insert user");
    user
}

pub async fn insert_comments(
    coll: &Collection<Comment>,
    commented_by: i32,
    commented_on: i32,
    count: i32,
) {
    let mut comments: Vec<Comment> = Vec::new();
    for n in 0..count {
        comments.push(Comment {
            comment_id: n,
            commented_on,
        });
    }
    // insert all in one go
    coll.insert_many(comments, None)
        .await
        .expect("Error generating posts");
}

pub async fn coll_count<T>(coll: &Collection<T>) -> u64 {
    coll.estimated_document_count(None)
        .await
        .expect("Error counting")
}

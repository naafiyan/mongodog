mod post;
mod user;

use mongowner::Schemable;
use user::User;
use post::Post;
use mongowner::mongo::options::{ClientOptions};
use mongowner::mongo::{Client, Collection, Database};
use mongowner::mongo::error::Result;
use mongowner::mongo::bson::doc;
use dotenv::dotenv;
use uuid::Uuid;

const DB_NAME: &str = "social";

// helper function to connect to MongoDB
async fn mongo_connect(uri: String) -> Result<(Database)> {
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database(DB_NAME))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // Replace the placeholder with your Atlas connection string
    let db = {
        let mongo_uri = std::env::var("MONGOURI")
            .expect("MONGOURI field must be set");
        mongo_connect(mongo_uri).await?
    };

    let test_user_a = User {
        user_id: Uuid::new_v4(),
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 2,
        email: "alice_bob".to_string(),
    };

    let post = Post {
        text: "Hello this is a post".to_string(),
        posted_by: test_user_a.user_id,
        date: "12th March 2023".to_string(),
    };

    println!("Posts collection name: {:?}", post.collection_name());

    // enforces that the repository i.e. collection is of type User
    let collection : Collection<User> = db.collection(test_user_a.collection_name());
    collection.insert_one(&test_user_a, None).await?;

    let found_user = collection
        .find_one(doc! { "username": "Alice" }, None)
        .await?
        .unwrap();

    Ok(())
}

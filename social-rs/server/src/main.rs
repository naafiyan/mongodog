mod post;
mod user;

use mongowner::Schemable;
use user::User;
use post::Post;
use mongowner::mongo::options::{ClientOptions};
use mongowner::mongo::{Client, Collection};
use mongowner::mongo::error::Result;
use mongowner::mongo::bson::doc;

#[tokio::main]
async fn main() -> Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    let db = client.database("social");

    let test_user_a = User {
        id: None,
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 2,
        email: "alice_bob".to_string(),
    };

    let post = Post {
        id: None,
        text: "Hello this is a post".to_string(),
        posted_by: 0,
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

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
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const DB_NAME: &str = "social";
const COLL_NAME: &str = "users"; // TODO: N - remove - just a placeholder for now
#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to social_rs")
}

#[get("/get_all_users")]
async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().body("Getting all users:")
}
///
/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection("users");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// TODO: N - indexes actually ensure that field is unique in MongoDB - could leverage this in
// mongowner
// Creates an index on the "username" field to force the values to be unique.
// async fn create_username_index(client: &Client) {
//     let options = IndexOptions::builder().unique(true).build();
//     let model = IndexModel::builder()
//         .keys(doc! { "username": 1 })
//         .options(options)
//         .build();
//     client
//         .database(DB_NAME)
//         .collection::<User>(COLL_NAME)
//         .create_index(model, None)
//         .await
//         .expect("creating an index should succeed");
// }
//
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

    println!("Posts collection name: {:?}", Post::collection_name());

    // enforces that the repository i.e. collection is of type User
    let collection : Collection<User> = db.collection(User::collection_name());
    collection.insert_one(&test_user_a, None).await?;

    let found_user = collection
        .find_one(doc! { "username": "Alice" }, None)
        .await?
        .unwrap();

    Ok(())
}

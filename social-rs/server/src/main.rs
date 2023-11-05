mod post;
mod user;
use std::vec;
use mongowner::Schemable;
use user::User;
use post::Post;
use mongowner::mongo::{Client, Collection, Database};
use mongowner::mongo::Cursor;
use mongowner::mongo::bson::doc;
use mongowner::mongo::options::ClientOptions;
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use uuid::Uuid;
use actix_web::{get, post, web, HttpResponse, HttpServer, App, Responder};

const DB_NAME: &str = "social";

#[get("/")]
async fn home() -> impl Responder {
    println!("Home page req");
    HttpResponse::Ok().body("Welcome to social_rs")
}

#[get("/get_all_users")]
async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(User::collection_name());
    let mut cursor = match collection.find(None, None).await {
        mongowner::mongo::error::Result::Ok(cursor) => cursor,
        mongowner::mongo::error::Result::Err(err) => panic!() // TODO: N - better error handling
    };
    let mut user_vec: Vec<User> = Vec::new();
    // TODO: N - loop through and add users

    HttpResponse::Ok().json(user_vec)
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(User::collection_name());
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
async fn add_user(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
    println!("Req received at /add-user");
    let collection = client.database(DB_NAME).collection(User::collection_name());
    println!("Getting user to add: {:?}", form.clone());
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Replace the placeholder with your Atlas connection string
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("MONGOURI field must be set"),
    };
    let client_options = ClientOptions::parse(uri).await.expect("URI failed to parse");
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options).expect("Client failed to initialize");
    // Send a ping to confirm a successful connection
    let db = client.database("test1");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(get_user)
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    
    // enforces that the repository i.e. collection is of type User
    let user_collection: Collection<User> = db.collection(User::collection_name());
    let post_collection: Collection<Post> = db.collection(Post::collection_name());

    // Clean users and posts repositories to begin
    user_collection.delete_many(doc! {}, None).await.expect("Clearing users failed.");
    post_collection.delete_many(doc! {}, None).await.expect("Clearing posts failed.");

    // Show we have empty repositories
    println!("Repository start state (no text if empty):");
    // let mut user_cursor = user_collection.find(None, None).await?;
    // while user_cursor.advance().await? {
    //     println!("{:?}", user_cursor.deserialize_current());
    // }
    // let mut post_cusor = post_collection.find(None, None).await?;
    // while post_cusor.advance().await? {
    //     println!("{:?}", post_cusor.deserialize_current());
    // }

    // impl User safe_delete for User {
    //     fn safe_delete(&self) {
    //         println!("safe deleting self");
    //         user_collection.delete_one(doc! { "user_id": self.user_id }, None).await?;
    //     }
    // }

    let test_user_a = User {
        user_id: mongowner::mongo::bson::uuid::Uuid::new(),
        username: "alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "of Wonderland".to_string(),
        age: 0,
        email: "alice_of_wonderland@brown.edu".to_string(),
    };

    let test_user_b = User {
        user_id: mongowner::mongo::bson::uuid::Uuid::new(),
        username: "bob".to_string(),
        first_name: "Bob".to_string(),
        last_name: "Ross".to_string(),
        age: 0,
        email: "bob_ross@brown.edu".to_string(),
    };

    let post_a = Post {
        text: "Hello this is a post".to_string(),
        posted_by: test_user_a.user_id.clone(),
        date: "12th March 2023".to_string(),
    };

    let post_b = Post {
        text: "Hello this is another post".to_string(),
        posted_by: test_user_a.user_id.clone(),
        date: "12th March 2023".to_string(),
    };

    let post_c = Post {
        text: "Hello this is a third post".to_string(),
        posted_by: test_user_b.user_id.clone(),
        date: "12th March 2023".to_string(),
    };

    // add users and posts 
    user_collection.insert_one(&test_user_a, None).await.expect("Inserting user a failed.");
    user_collection.insert_one(&test_user_b, None).await.expect("Inserting user b failed.");

    post_collection.insert_one(&post_a, None).await.expect("Inserting post a failed.");
    post_collection.insert_one(&post_b, None).await.expect("Inserting post b failed.");
    post_collection.insert_one(&post_c, None).await.expect("Inserting post c failed.");

    println!("New set of users and posts:");
    let mut user_cursor = user_collection.find(None, None).await.expect("");
    while user_cursor.advance().await.expect("") {
        println!("{:?}", user_cursor.deserialize_current().expect(""));
    }
    let mut post_cusor = post_collection.find(None, None).await.expect("");
    while post_cusor.advance().await.expect("") {
        println!("{:?}", post_cusor.deserialize_current().expect(""));
    }



    // println!("Test user a username {:?}", test_user_a.username);
    // let found_post = post_collection
    //     .find_one(doc! { f!(text in Post): "Hello this is a post" }, None)
    //     .await?
    //     .unwrap();
    
    // println!("Found post: {:?}", found_post);
    // Post::cascade_delete(&test_post);
    // collection.insert_one(&test_user_a, None).await?;
    // let _found_user = collection
    //     .find_one(doc! { "username": "Alice" }, None)
    //     .await?
    //     .unwrap();

    Ok(())
}

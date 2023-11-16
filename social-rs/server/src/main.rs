mod post;
mod user;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongowner::delete::safe_delete;
use mongowner::mongo::bson::doc;
use mongowner::mongo::{Client, Collection, Database};
use mongowner::Schemable;
use petgraph::{algo::is_cyclic_directed, graphmap, Directed};
use std::fs;
use std::io::Read;
use user::User;
use post::Post;
use futures::{TryStreamExt, StreamExt};


const DB_NAME: &str = "social";

#[get("/")]
async fn home() -> impl Responder {
    println!("Home page req");
    HttpResponse::Ok().body("Welcome to social_rs")
}


#[get("/clear_users")]
async fn clear_users(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(User::collection_name());
    collection.delete_many(doc! {}, None).await.expect("Clearing users failed.");
    HttpResponse::Ok().body("Users cleared")
}

#[get("/get_all_users")]
async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
    println!("DEBUG: get_all_users");
    let collection: Collection<User> = client.database(DB_NAME).collection(User::collection_name());
    let mut users_cursor = match collection.find(None, None).await {
        mongowner::mongo::error::Result::Ok(cursor) => cursor,
        mongowner::mongo::error::Result::Err(err) => panic!("Failed in cursor loop"), // TODO: N - better error handling
    };

    let mut users: Vec<User> = Vec::new();
    while let Some(doc) = users_cursor.next().await {
        match doc {
            Ok(user) => {
                users.push(user);
            }
            Err(e) => {
                eprintln!("Error fetching document: {}", e);
            }
        }
    }
    HttpResponse::Ok().json(users)
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

    // ----- temp: this should not be explicit code! ----
    // load the graph from the file and validate it
    let mut file = fs::File::open("./data/graph.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut graph: graphmap::GraphMap<&str, &str, Directed> = match serde_json::from_str(&contents)
    {
        Ok(g) => g,
        Err(_) => graphmap::GraphMap::new(),
    };
    let graph = graph.into_graph::<u32>();
    println!("DEBUG: ownership graph: {:?}", &graph);
    println!(
        "VALIDATION: graph is not cyclic: {:?}",
        !is_cyclic_directed(&graph)
    );

    // --------------------------------------------------

    // Replace the placeholder with your Atlas connection string
    let uri = std::env::var("MONGOURI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    let user = User {
        user_id: mongowner::mongo::bson::Uuid::new(),
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };
    println!("Attempting to call safe_delete");
    safe_delete(&user, &client.database("socials"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(get_user)
            .service(clear_users)
            .service(get_all_users)
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

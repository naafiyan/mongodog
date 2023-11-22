mod comment;
mod post;
mod user;

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use mongowner::delete::safe_delete;
use mongowner::mongo::bson::doc;
use mongowner::mongo::{Client, Collection, Database};
use mongowner::Schemable;
use petgraph::{algo::is_cyclic_directed, graphmap, Directed};
use post::Post;
use user::User;

const DB_NAME: &str = "social";

#[get("/")]
async fn home() -> impl Responder {
    println!("Home page req");
    HttpResponse::Ok().body("Welcome to social_rs")
}

#[delete("/clear_users")]
async fn clear_users(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(User::collection_name());
    collection
        .delete_many(doc! {}, None)
        .await
        .expect("Clearing users failed.");
    HttpResponse::Ok().body("Users cleared")
}

#[delete("/clear_posts")]
async fn clear_posts(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Post> = client.database(DB_NAME).collection(Post::collection_name());
    collection
        .delete_many(doc! {}, None)
        .await
        .expect("Clearing posts failed.");
    HttpResponse::Ok().body("Posts cleared")
}

#[get("/get_all_users")]
async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
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
                HttpResponse::InternalServerError().body(e.to_string());
            }
        }
    }
    HttpResponse::Ok().json(users)
}

#[get("/get_all_posts")]
async fn get_all_posts(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Post> = client.database(DB_NAME).collection(Post::collection_name());
    let mut posts_cursor = match collection.find(None, None).await {
        mongowner::mongo::error::Result::Ok(cursor) => cursor,
        mongowner::mongo::error::Result::Err(err) => panic!("Failed in cursor loop"), // TODO: N - better error handling
    };

    let mut posts: Vec<Post> = Vec::new();
    while let Some(doc) = posts_cursor.next().await {
        match doc {
            Ok(post) => {
                posts.push(post);
            }
            Err(e) => {
                HttpResponse::InternalServerError().body(e.to_string());
            }
        }
    }
    HttpResponse::Ok().json(posts)
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection("users");
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

/// Gets posts from a certain user
#[get("/get_posts_for_user/{username}")]
async fn get_posts_for_user(
    client: web::Data<Client>,
    username: web::Path<String>,
) -> HttpResponse {
    let username = username.into_inner();
    let users_collection: Collection<User> =
        client.database(DB_NAME).collection(User::collection_name());
    match users_collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => {
            let posts_collection: Collection<Post> =
                client.database(DB_NAME).collection(Post::collection_name());
            let mut posts_cursor = match posts_collection
                .find(doc! { "posted_by": user.user_id }, None)
                .await
            {
                mongowner::mongo::error::Result::Ok(cursor) => cursor,
                mongowner::mongo::error::Result::Err(err) => panic!("Failed in cursor loop"), // TODO: N - better error handling
            };
            let mut posts: Vec<Post> = Vec::new();
            while let Some(doc) = posts_cursor.next().await {
                match doc {
                    Ok(post) => {
                        posts.push(post);
                    }
                    Err(e) => {
                        HttpResponse::InternalServerError().body(e.to_string());
                    }
                }
            }
            HttpResponse::Ok().json(posts)
        }
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
    let collection = client.database(DB_NAME).collection("users");
    println!("Getting user to add: {:?}", form.clone());
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Adds a new user to the "users" collection in the database.
#[post("/add_post")]
async fn add_post(client: web::Data<Client>, form: web::Json<Post>) -> HttpResponse {
    println!("Req received at /add-post");
    let collection = client.database(DB_NAME).collection(Post::collection_name());
    println!("Getting post to add: {:?}", form.clone());
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Post added"),
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

    // ----- temp: graph validation code ----
    // load the graph from the file and validate it
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dir_path = Path::new(&out_dir);
    // let graph_path = dir_path.join("graph.json");
    // let mut file = fs::File::open(graph_path)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // let graph: graphmap::GraphMap<&str, &str, Directed> = match serde_json::from_str(&contents)
    // {
    //     Ok(g) => g,
    //     Err(_) => graphmap::GraphMap::new(),
    // };
    // let graph = graph.into_graph::<u32>();
    // println!("DEBUG: ownership graph: {:?}", &graph);
    // println!(
    //     "VALIDATION: graph is not cyclic: {:?}",
    //     !is_cyclic_directed(&graph)
    // );

    // --------------------------------------------------

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
    let post = Post {
        post_id: mongowner::mongo::bson::Uuid::new(),
        text: "hello world".to_string(),
        posted_by: user.user_id,
        date: "2023-11-08".to_string(),
    };

    // println!("Attempting to call safe_delete");
    // let posts_coll = client.database("socials").collection::<Post>("posts");
    // posts_coll.insert_one(post, None).await.unwrap();
    // let users_coll = client.database("socials").collection::<User>("users");
    // users_coll.insert_one(&user, None).await.unwrap();
    // safe_delete(user, &client.database("socials"))
    //     .await
    //     .unwrap();
    // println!("safe-deleted");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(add_post)
            .service(get_user)
            .service(clear_users)
            .service(clear_posts)
            .service(get_all_users)
            .service(get_all_posts)
            .service(get_posts_for_user)
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
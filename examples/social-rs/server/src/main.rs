mod comment;
mod post;
mod user;

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use mongowner::delete::safe_delete;
use mongowner::mongo::bson::doc;
use mongowner::mongo::{Client, Collection, Database};
use mongowner::Schemable;
use petgraph::{algo::is_cyclic_directed, graphmap, Directed};
use post::Post;
use user::User;
use comment::Comment;

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

#[delete("/delete_post/{post_id}")]
async fn delete_post(client: web::Data<Client>,  post_id: web::Path<String>) -> HttpResponse {
    let post_id = post_id.into_inner();
    let database = client.database(DB_NAME);
    let collection: Collection<Post> = database.collection(Post::collection_name());
    let post = match collection
        .find_one(doc! { "post_id": post_id.clone().to_string().parse::<i32>().unwrap() }, None)
        .await
    {
        Ok(Some(post)) => post,
        Ok(None) => {
            return HttpResponse::NotFound().body(format!("No post found with post_id {post_id}"))
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let result = safe_delete(post, &database).await;
    HttpResponse::Ok().body("Post deletion successful")
}


#[delete("/delete_user/{user_id}")]
async fn delete_user(client: web::Data<Client>,  user_id: web::Path<String>) -> HttpResponse {
    let user_id = user_id.into_inner();
    let database = client.database(DB_NAME);
    let collection: Collection<User> = database.collection(User::collection_name());
    let user = match collection
        .find_one(doc! { "user_id": user_id.clone().to_string().parse::<i32>().unwrap() }, None)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().body(format!("No user found with user_id {user_id}"))
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let result = safe_delete(user, &database).await;
    HttpResponse::Ok().body("User deletion successful")
}

#[delete("/delete_comment/{comment_id}")]
async fn delete_comment(client: web::Data<Client>,  comment_id: web::Path<String>) -> HttpResponse {
    let comment_id = comment_id.into_inner();
    let database = client.database(DB_NAME);
    let collection: Collection<Comment> = database.collection(Comment::collection_name());
    let comment = match collection
        .find_one(doc! { "comment_id": comment_id.clone().to_string().parse::<i32>().unwrap() }, None)
        .await
    {
        Ok(Some(comment)) => comment,
        Ok(None) => {
            return HttpResponse::NotFound().body(format!("No comment found with id {comment_id}"))
        }
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let result = safe_delete(comment, &database).await;
    HttpResponse::Ok().body("Comment deletion successful")
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

/// Adds a new post to the "posts" collection in the database.
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

/// Adds a new post to the "posts" collection in the database.
#[post("/add_comment")]
async fn add_comment(client: web::Data<Client>, form: web::Json<Comment>) -> HttpResponse {
    println!("Req received at /add-comment");
    let collection = client.database(DB_NAME).collection(Comment::collection_name());
    println!("Getting post to add: {:?}", form.clone());
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Comment added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


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
        user_id: 2, // temporarily using a u8 here
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };
    // let post = Post {
    //     post_id: 4,
    //     text: "hello world".to_string(),
    //     posted_by: user.user_id,
    //     date: "2023-11-08".to_string(),
    // };

    // println!("Attempting to call safe_delete");
    // let posts_coll = client.database("socials").collection::<Post>("posts");
    // posts_coll.insert_one(post, None).await.unwrap();
    // let users_coll = client.database("socials").collection::<User>("users");
    // users_coll.insert_one(&user, None).await.unwrap();


    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(add_post)
            .service(add_comment)
            .service(get_user)
            .service(clear_users)
            .service(clear_posts)
            .service(get_all_users)
            .service(get_all_posts)
            .service(delete_post)
            .service(delete_user)
            .service(delete_comment)
            .service(get_posts_for_user)
            .service(home)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}

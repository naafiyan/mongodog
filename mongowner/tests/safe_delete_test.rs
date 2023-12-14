use mongowner::delete::safe_delete;
use std::u32;

use fake::faker::boolean::en::Boolean;
use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::lorem::en::{Paragraph, Word};
use fake::faker::name::en::Name;
use fake::Fake;
use mongodb::{Client, Collection, Database};
use mongowner::{Schema, Schemable};
use rand::random;
use serde::{Deserialize, Serialize};

/// A set of schemas meant to encompass different ownership relations and structures

#[derive(Schema, Serialize, Deserialize)]
#[data_subject]
#[collection(users)]
pub struct User {
    #[index]
    id: u32,
    username: String,
    name: String,
    email: String,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(posts)]
pub struct Post {
    #[index]
    id: u32,
    #[owned_by(users, id)]
    posted_by: u32,
    text: String,
    date: String,
}

impl Post {
    pub fn new(id: u32, posted_by: u32, text: String, date: String) -> Post {
        Post {
            id,
            posted_by,
            text,
            date,
        }
    }
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(comments)]
pub struct Comment {
    #[index]
    id: u32,
    #[owned_by(users, id)]
    commented_by: u32,
    #[owned_by(posts, id)]
    parent_post: u32,
    text: String,
    date: String,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(productives)]
pub struct Productive {
    #[index]
    id: u32,
    #[owned_by(users, id)]
    rated_by: u32,
    #[owned_by(comments, id)]
    comment: u32,
    is_productive: bool,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(attachments)]
pub struct Attachment {
    #[owned_by(users, id)]
    user: u32,
    #[owned_by(posts, id)]
    post: u32,
    #[owned_by(comments, id)]
    comment: u32,
    #[index]
    resource_id: u32,
    date: String,
    moderated_by: u32,
}

#[derive(Schema, Serialize, Deserialize)]
#[data_subject]
#[collection(mediamods)]
pub struct MediaMod {
    #[index]
    id: u32,
    media_group: String,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(modposts)]
pub struct ModPost {
    #[index]
    id: u32,
    #[owned_by(mediamods, id)]
    moderator: u32,
    text: String,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(modresources)]
pub struct ModResource {
    #[index]
    id: u32,
    #[owned_by(mediamods, id)]
    moderator: u32,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(mresource1s)]
pub struct MResource1 {
    #[index]
    id: u32,
    r1: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(mresources2)]
pub struct MResource2 {
    #[index]
    id: u32,
    r2: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
}

#[derive(Schema, Serialize, Deserialize)]
#[collection(mresources3)]
pub struct MResource3 {
    #[index]
    id: u32,
    r3: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
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

pub async fn insert_posts(coll: &Collection<Post>, owner_id: u32, count: u32) {
    let mut posts: Vec<Post> = Vec::new();
    for n in 0..count {
        posts.push(Post {
            id: n,
            posted_by: owner_id,
            text: Paragraph(0..2).fake(),
            date: "Dec 13, 2023".to_string(),
        });
    }
    // insert all in one go
    coll.insert_many(posts, None)
        .await
        .expect("Error generating posts");
}

pub async fn insert_user(coll: &Collection<User>, user_id: u32) -> User {
    let user = User {
        id: user_id,
        username: Username().fake(),
        name: Name().fake(),
        email: FreeEmail().fake(),
    };
    coll.insert_one(&user, None)
        .await
        .expect("Failed to insert user");
    user
}

pub async fn insert_comments(
    coll: &Collection<Comment>,
    commented_by: u32,
    commented_on: u32,
    count: u32,
) {
    let mut comments: Vec<Comment> = Vec::new();
    for n in 0..count {
        comments.push(Comment {
            id: n,
            parent_post: commented_on,
            commented_by,
            text: Word().fake(),
            date: "Dec 14, 2023".to_string(),
        });
    }
    // insert all in one go
    coll.insert_many(comments, None)
        .await
        .expect("Error generating comments");
}

pub async fn insert_productives(
    coll: &Collection<Productive>,
    rated_by: u32,
    comment: u32,
    count: u32,
) {
    let mut productives: Vec<Productive> = Vec::new();
    for n in 0..count {
        productives.push(Productive {
            id: n,
            rated_by,
            comment,
            is_productive: Boolean(1).fake(),
        });
    }
    coll.insert_many(productives, None)
        .await
        .expect("Error generating Productives");
}

pub async fn insert_attachments(
    coll: &Collection<Attachment>,
    user: u32,
    post: u32,
    comment: u32,
    count: u32,
) {
    let mut v: Vec<Attachment> = Vec::new();
    for n in 0..count {
        v.push(Attachment {
            user,
            post,
            comment,
            resource_id: n,
            date: "Dec 14, 2023".to_string(),
            moderated_by: 0,
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating attachments");
}

pub async fn insert_mediamods(coll: &Collection<MediaMod>, count: u32) {
    let mut v: Vec<MediaMod> = Vec::new();
    for n in 0..count {
        v.push(MediaMod {
            id: n,
            media_group: Word().fake(),
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating attachments");
}

pub async fn insert_modposts(coll: &Collection<ModPost>, moderator: u32, count: u32) {
    let mut v: Vec<ModPost> = Vec::new();
    for n in 0..count {
        v.push(ModPost {
            id: n,
            moderator,
            text: Paragraph(0..4).fake(),
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating attachments");
}

pub async fn insert_modresources(coll: &Collection<ModResource>, moderator: u32, count: u32) {
    let mut v: Vec<ModResource> = Vec::new();
    for n in 0..count {
        v.push(ModResource { id: n, moderator });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating modresources");
}

pub async fn insert_mresources1(coll: &Collection<MResource1>, parent_res: u32, count: u32) {
    let mut v: Vec<MResource1> = Vec::new();
    for n in 0..count {
        v.push(MResource1 {
            id: n,
            r1: Word().fake(),
            parent_res,
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating mresources1");
}

pub async fn insert_mresources2(coll: &Collection<MResource2>, parent_res: u32, count: u32) {
    let mut v: Vec<MResource2> = Vec::new();
    for n in 0..count {
        v.push(MResource2 {
            id: n,
            r2: Word().fake(),
            parent_res,
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating mresources2");
}

pub async fn insert_mresources3(coll: &Collection<MResource3>, parent_res: u32, count: u32) {
    let mut v: Vec<MResource3> = Vec::new();
    for n in 0..count {
        v.push(MResource3 {
            id: n,
            r3: Word().fake(),
            parent_res,
        });
    }
    coll.insert_many(v, None)
        .await
        .expect("Error generating mresources3");
}

pub async fn coll_count<T>(coll: &Collection<T>) -> u64 {
    coll.estimated_document_count(None)
        .await
        .expect("Error counting")
}

pub fn set_graph_name() {
    // let graph_name = format!("graph{}.json", Uuid::new_v4().to_string());
    // std::env::set_var("GRAPH_NAME", &graph_name);
    // println!("TEST DEUBG: GRAPH_NAME is {}", graph_name);
}
// tests if safe_delete works when 1 user owns 1 post
#[tokio::test]
async fn safe_delete_single() {
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let user = insert_user(&user_coll, 0).await;

    insert_posts(&post_coll, 0, 10).await;
    assert_eq!(10, coll_count(&post_coll).await);

    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, coll_count(&user_coll).await);
    assert_eq!(0, coll_count(&post_coll).await);

    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_post_comment() {
    set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let user = insert_user(&user_coll, user_id).await;

    insert_posts(&post_coll, user_id, 10).await;
    // TODO: do all the inserts is in 1 operation
    assert_eq!(10, coll_count(&post_coll).await);

    insert_comments(&comment_coll, user_id, 2, 100).await;
    assert_eq!(100, coll_count(&comment_coll).await);

    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, coll_count::<User>(&user_coll).await);
    assert_eq!(0, coll_count::<Post>(&post_coll).await);
    assert_eq!(0, coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// 2 Users, UserA owns Posts [0, 4] and UserB owns Posts [5, 9]
// Post2 owns Comments [0, 39] and Post7 Owns Comments [40, 99]
#[tokio::test]
async fn safe_delete_multiple_users() {
    set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let a_id = 0;
    let a = insert_user(&user_coll, a_id).await;
    let b_id = 1;
    let _ = insert_user(&user_coll, a_id).await;

    insert_posts(&post_coll, a_id, 5).await;
    insert_posts(&post_coll, b_id, 5).await;
    assert_eq!(10, coll_count(&post_coll).await);

    insert_comments(&comment_coll, 3, 2, 40).await;
    insert_comments(&comment_coll, 3, 7, 60).await;
    assert_eq!(100, coll_count(&comment_coll).await);

    safe_delete(a, &db).await.expect("Error safe deleting");

    assert_eq!(1, coll_count::<User>(&user_coll).await);
    assert_eq!(5, coll_count::<Post>(&post_coll).await);
    assert_eq!(60, coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_multiple_owners() {
    set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let user = insert_user(&user_coll, user_id).await;

    // User0 owns 10 posts with ids in range [0, 9]
    insert_posts(&post_coll, user_id, 10).await;
    assert_eq!(10, coll_count(&post_coll).await);

    // User0 owns 100 comments on Post100
    insert_comments(&comment_coll, user_id, 100, 100).await;
    assert_eq!(100, coll_count(&comment_coll).await);

    // User1 owns 100 comments on Post2
    insert_comments(&comment_coll, 1, 2, 100).await;
    assert_eq!(200, coll_count(&comment_coll).await);

    // User1 owns 100 comments on Post11
    insert_comments(&comment_coll, 1, 11, 100).await;
    assert_eq!(300, coll_count(&comment_coll).await);
    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, coll_count::<User>(&user_coll).await);
    assert_eq!(0, coll_count::<Post>(&post_coll).await);

    // all comments belonging to User0 should be deleted + all comments belonging to Posts by User0
    assert_eq!(100, coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_post() {
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let _ = insert_user(&user_coll, user_id).await;

    // User0 owns 10 posts with ids in range [0, 9]
    insert_posts(&post_coll, user_id, 10).await;
    assert_eq!(10, coll_count(&post_coll).await);

    // User0 owns 100 comments on Post2
    insert_comments(&comment_coll, 0, 2, 100).await;
    assert_eq!(100, coll_count(&comment_coll).await);

    insert_comments(&comment_coll, 0, 3, 100).await;
    assert_eq!(200, coll_count(&comment_coll).await);
    let post = Post::new(2, 0, "hello".to_string(), "Dec 13, 2023".to_string());

    // safe_deleting Post2
    safe_delete(post, &db).await.expect("Error safe deleting");

    assert_eq!(1, coll_count::<User>(&user_coll).await);
    assert_eq!(9, coll_count::<Post>(&post_coll).await);

    // all comments belonging to User0 should be deleted + all comments belonging to Posts by User0
    assert_eq!(100, coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// TODO: add safe_delete_large test that has a total of 100000 (100K) documents spread across the collections with 1 user owning 10000 posts each with 10 comments
#[tokio::test]
async fn safe_delete_large() {
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    // get all the other collections
    let productives_coll = db.collection::<Productive>(Productive::collection_name());
    let attachment_coll = db.collection::<Attachment>(Attachment::collection_name());
    let mediamod_coll = db.collection::<MediaMod>(MediaMod::collection_name());
    let modpost_coll = db.collection::<ModPost>(ModPost::collection_name());
    let modresource_coll = db.collection::<ModResource>(ModResource::collection_name());
    let mresource1_coll = db.collection::<MResource1>(MResource1::collection_name());
    let mresource2_coll = db.collection::<MResource2>(MResource2::collection_name());
    let mresource3_coll = db.collection::<MResource3>(MResource3::collection_name());

    let user_id = 0;
    let _ = insert_user(&user_coll, user_id).await;

    // User0 owns 10k posts with ids in range [0, 9999]
    insert_posts(&post_coll, user_id, 10000).await;
    assert_eq!(10000, coll_count(&post_coll).await);

    // User0 owns 100 comments on Post2
    insert_comments(&comment_coll, 0, 2, 100).await;
    assert_eq!(100, coll_count(&comment_coll).await);

    insert_comments(&comment_coll, 0, 3, 100).await;
    assert_eq!(200, coll_count(&comment_coll).await);

     // use productives_coll
    insert_productives(&productives_coll, 0, 2, 100).await;
}

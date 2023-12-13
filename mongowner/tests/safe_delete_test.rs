mod common;
use common::{init_test_db, teardown_db, Comment, Post, User};
use mongowner::delete::safe_delete;
use mongowner::Schemable;

// tests if safe_delete works when 1 user owns 1 post
#[tokio::test]
async fn safe_delete_single() {
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let user = common::insert_user(&user_coll, 0).await;

    common::insert_posts(&post_coll, 0, 10).await;
    assert_eq!(10, common::coll_count(&post_coll).await);

    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, common::coll_count(&user_coll).await);
    assert_eq!(0, common::coll_count(&post_coll).await);

    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_post_comment() {
    common::set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let user = common::insert_user(&user_coll, user_id).await;

    common::insert_posts(&post_coll, user_id, 10).await;
    // TODO: do all the inserts is in 1 operation
    assert_eq!(10, common::coll_count(&post_coll).await);

    common::insert_comments(&comment_coll, user_id, 2, 100).await;
    assert_eq!(100, common::coll_count(&comment_coll).await);

    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, common::coll_count::<User>(&user_coll).await);
    assert_eq!(0, common::coll_count::<Post>(&post_coll).await);
    assert_eq!(0, common::coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// 2 Users, UserA owns Posts [0, 4] and UserB owns Posts [5, 9]
// Post2 owns Comments [0, 39] and Post7 Owns Comments [40, 99]
#[tokio::test]
async fn safe_delete_multiple_users() {
    common::set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let a_id = 0;
    let a = common::insert_user(&user_coll, a_id).await;
    let b_id = 1;
    let _ = common::insert_user(&user_coll, a_id).await;

    common::insert_posts(&post_coll, a_id, 5).await;
    common::insert_posts(&post_coll, b_id, 5).await;
    assert_eq!(10, common::coll_count(&post_coll).await);

    common::insert_comments(&comment_coll, 3, 2, 40).await;
    common::insert_comments(&comment_coll, 3, 7, 60).await;
    assert_eq!(100, common::coll_count(&comment_coll).await);

    safe_delete(a, &db).await.expect("Error safe deleting");

    assert_eq!(1, common::coll_count::<User>(&user_coll).await);
    assert_eq!(5, common::coll_count::<Post>(&post_coll).await);
    assert_eq!(60, common::coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_multiple_owners() {
    common::set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let user = common::insert_user(&user_coll, user_id).await;

    // User0 owns 10 posts with ids in range [0, 9]
    common::insert_posts(&post_coll, user_id, 10).await;
    assert_eq!(10, common::coll_count(&post_coll).await);

    // User0 owns 100 comments on Post100
    common::insert_comments(&comment_coll, user_id, 100, 100).await;
    assert_eq!(100, common::coll_count(&comment_coll).await);

    // User1 owns 100 comments on Post2
    common::insert_comments(&comment_coll, 1, 2, 100).await;
    assert_eq!(200, common::coll_count(&comment_coll).await);

    // User1 owns 100 comments on Post11
    common::insert_comments(&comment_coll, 1, 11, 100).await;
    assert_eq!(300, common::coll_count(&comment_coll).await);
    safe_delete(user, &db).await.expect("Error safe deleting");

    assert_eq!(0, common::coll_count::<User>(&user_coll).await);
    assert_eq!(0, common::coll_count::<Post>(&post_coll).await);

    // all comments belonging to User0 should be deleted + all comments belonging to Posts by User0
    assert_eq!(100, common::coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

// Comment owned by Post owned by User
#[tokio::test]
async fn safe_delete_post() {
    common::set_graph_name();
    let db = init_test_db().await.expect("Error with init test db");
    let user_coll = db.collection::<User>(User::collection_name());
    let post_coll = db.collection::<Post>(Post::collection_name());
    let comment_coll = db.collection::<Comment>(Comment::collection_name());
    let user_id = 0;
    let _ = common::insert_user(&user_coll, user_id).await;

    // User0 owns 10 posts with ids in range [0, 9]
    common::insert_posts(&post_coll, user_id, 10).await;
    assert_eq!(10, common::coll_count(&post_coll).await);

    // User0 owns 100 comments on Post2
    common::insert_comments(&comment_coll, 0, 2, 100).await;
    assert_eq!(100, common::coll_count(&comment_coll).await);

    common::insert_comments(&comment_coll, 0, 3, 100).await;
    assert_eq!(200, common::coll_count(&comment_coll).await);

    let post = Post {
        post_id: 2,
        posted_by: 0,
    };
    // safe_deleting Post2
    safe_delete(post, &db).await.expect("Error safe deleting");

    assert_eq!(1, common::coll_count::<User>(&user_coll).await);
    assert_eq!(9, common::coll_count::<Post>(&post_coll).await);

    // all comments belonging to User0 should be deleted + all comments belonging to Posts by User0
    assert_eq!(100, common::coll_count::<Comment>(&comment_coll).await);
    teardown_db(&db).await;
}

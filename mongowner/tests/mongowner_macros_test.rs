use mongowner::{Schema, Schemable};

#[test]
fn implements_schemable() {
    #[derive(Schema)]
    #[collection(users)]
    #[data_subject]
    pub struct User {
        #[index]
        pub user_id: mongowner::mongo::bson::uuid::Uuid,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub email: String,
    }

    let user = User {
        user_id: mongowner::mongo::bson::Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .unwrap(),
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };

    assert_eq!("users", User::collection_name());
    assert_eq!("user_id", User::index_name());
    assert_eq!(
        "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
        user.index_value().to_string()
    );
}

#[test]
fn implements_schemable_owned_by() {
    #[derive(Schema)]
    #[collection(users)]
    #[data_subject]
    pub struct User {
        #[index]
        pub user_id: mongowner::mongo::bson::uuid::Uuid,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub email: String,
    }

    let user = User {
        user_id: mongowner::mongo::bson::Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .unwrap(),
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };

    #[derive(Schema)]
    #[collection(posts)]
    pub struct Post {
        #[index]
        pub post_id: mongowner::mongo::bson::uuid::Uuid,
        pub text: String,
        #[owned_by(users, user_id)]
        pub posted_by: mongowner::mongo::bson::uuid::Uuid,
        pub date: String,
    }
    let post = Post {
        post_id: mongowner::mongo::bson::Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .unwrap(),
        text: "hello world".to_string(),
        posted_by: user.user_id,
        date: "2023-11-08".to_string(),
    };
    assert_eq!("posts", Post::collection_name());
    assert_eq!("post_id", Post::index_name());
    assert_eq!(
        "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
        post.index_value().to_string()
    );
}
#[test]
fn different_index_types() {
    #[derive(Schema)]
    #[collection(users)]
    #[data_subject]
    pub struct User {
        #[index]
        pub user_id: &'static str,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub email: String,
    }

    let user = User {
        user_id: "ABCDEFG",
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };

    assert_eq!("users", User::collection_name());
    assert_eq!("user_id", User::index_name());
    assert_eq!("ABCDEFG", user.index_value().to_string());
    #[derive(Schema)]
    #[collection(posts)]
    pub struct Post {
        #[index]
        pub post_id: u8,
        pub text: String,
        #[owned_by(users, user_id)]
        pub posted_by: &'static str,
        pub date: String,
    }
    let post = Post {
        post_id: 0,
        text: "hello world".to_string(),
        posted_by: user.user_id,
        date: "2023-11-08".to_string(),
    };
    assert_eq!(0, post.index_value());
}

#[test]
fn multiple_schemas() {
    #[derive(Schema)]
    #[collection(users)]
    #[data_subject]
    pub struct User {
        #[index]
        pub user_id: mongowner::mongo::bson::uuid::Uuid,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub email: String,
    }

    let user = User {
        user_id: mongowner::mongo::bson::Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .unwrap(),
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };

    #[derive(Schema)]
    #[collection(posts)]
    pub struct Post {
        #[index]
        pub post_id: mongowner::mongo::bson::uuid::Uuid,
        pub text: String,
        #[owned_by(users, user_id)]
        pub posted_by: mongowner::mongo::bson::uuid::Uuid,
        pub date: String,
    }
    let post = Post {
        post_id: mongowner::mongo::bson::Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")
            .unwrap(),
        text: "hello world".to_string(),
        posted_by: user.user_id,
        date: "2023-11-08".to_string(),
    };
    assert_eq!("posts", Post::collection_name());
    assert_eq!("post_id", Post::index_name());
    assert_eq!(
        "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
        post.index_value().to_string()
    );
}

#[test]
fn multiple_owned_by() {
    #[derive(Schema)]
    #[collection(users)]
    #[data_subject]
    pub struct User {
        #[index]
        pub user_id: &'static str,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub email: String,
    }

    let user = User {
        user_id: "ABCDEFG",
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 20,
        email: "alice_bob@brown.edu".to_string(),
    };

    assert_eq!("users", User::collection_name());
    assert_eq!("user_id", User::index_name());
    assert_eq!("ABCDEFG", user.index_value().to_string());
    #[derive(Schema)]
    #[collection(posts)]
    pub struct Post {
        #[index]
        pub post_id: u8,
        pub text: String,
        #[owned_by(users, user_id)]
        pub posted_by: &'static str,
        pub date: String,
    }
    let post = Post {
        post_id: 0,
        text: "hello world".to_string(),
        posted_by: user.user_id,
        date: "2023-11-08".to_string(),
    };
    assert_eq!(0, post.index_value());

    #[derive(Schema)]
    #[collection(comments)]
    pub struct Comment {
        #[index]
        pub comment_id: u8,
        #[owned_by(users, user_id)]
        pub commented_by: &'static str,
        #[owned_by(posts, post_id)]
        pub commented_on: u8,
        pub date: String,
    }
}

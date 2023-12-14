// mod common;
// use mongowner::Schemable;
//
// #[test]
// fn implements_schemable() {
//     let user = common::User {
//         user_id: 0,
//         username: "alice".to_string(),
//     };
//
//     assert_eq!("users", common::User::collection_name());
//     assert_eq!("user_id", common::User::index_name());
//     assert_eq!(0, user.index_value());
// }
//
// #[test]
// fn implements_schemable_owned_by() {
//     let post = common::Post {
//         post_id: 0,
//         posted_by: 0,
//     };
//     assert_eq!("posts", common::Post::collection_name());
//     assert_eq!("post_id", common::Post::index_name());
// }

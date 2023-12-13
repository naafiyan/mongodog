use std::u32;

use mongodb::{Client, Collection, Database};
use mongowner::{Schema, Schemable};
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A set of schemas meant to encompass different ownership relations and structures

#[data_subject]
struct User {
    id: u32,
    username: String,
    name: String,
    email: String,
}

struct Post {
    id: u32,
    #[owned_by(users, id)]
    posted_by: u32,
    text: String,
    date: String,
}

struct Comment {
    id: u32,
    #[owned_by(users, id)]
    posted_by: u32,
    #[owned_by(posts, id)]
    parent_post: u32,
    text: String,
    date: String,
}

struct Productive {
    id: u32,
    rated_by: u32,
    #[owned_by(comments, id)]
    comment: u32,
    is_productive: bool,
}

struct Attachment {
    #[owned_by(users, id)]
    user: u32,
    #[owned_by(posts, id)]
    #[owned_by(comments, id)]
    resource_id: u32,
    date: String,
    moderated_by: u32,
}

#[data_subject]
struct MediaMod {
    id: u32,
    media_group: String,
}

struct ModPost {
    id: u32,
    #[owned_by(mediamods, id)]
    moderator: u32,
    text: String,
}

struct ModResource {
    id: u32,
    #[owned_by(mediamods, id)]
    moderator: u32,
}

struct MResource1 {
    id: u32,
    r1: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
}

struct MResource2 {
    id: u32,
    r2: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
}

struct MResource3 {
    id: u32,
    r3: String,
    #[owned_by(modresources, id)]
    parent_res: u32,
}

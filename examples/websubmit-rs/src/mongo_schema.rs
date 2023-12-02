use mongowner::{Schema, Schemable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
#[data_subject]
pub struct User {
    #[index]
    email: String,
    apikey: String,
    is_admin: u8,
    // TODO: N - enforce unique API KEY
}

// N - relationship here not relevant to our project
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Lecture {
    id: u32,
    label: String,
}

// N - relationship here not relevant to our project
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Question {
    id: u32,         // TODO: N - this should be an index but oh well
    lecture_id: u32, // FK but NOT owned_by
    question_number: u32,
    question: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(answers)]
pub struct Answer {
    #[index]
    id: u32,
    #[owned_by(users, email)]
    email: String, // FK but NOT owned_by
    question_id: u32,
    answer: String,
    submitted_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(presenters)]
pub struct Presenter {
    #[index]
    id: u32,
    #[owned_by(users, email)]
    email: String, // FK but NOT owned_by
    lecture_id: u32,
}

# Mongodog
Mongodog is a final project for Brown University's CS2390: Privacy-Conscious Computer Systems developed by Naafiyan Ahmed, Swetabh Changkakoti and Lucas Gelfond. We aim to improve developer experience and ensure correct deletion of data in a MongoDB database. It consists a Rust crate called `mongowner` as well as example implementations of this crate on applications.

## Mongowner
We provide a Rust crate called `mongowner` that allows for easy-to-implement data ownership modelling and safe cascade deletion for MongoDB. We will publish this crate once we feel it is feature-ready and stable.

Building the crate:
```
cd mongowner
cargo build
```

Example of struct annotations for 2 schemas - `User` and `Post` to model the ownership relationship, i.e. `Post` is owned by `User`
```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(users)]
#[data_subject]
pub struct User {
    #[index]
    pub user_id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}
```
```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Schema)]
#[collection(posts)]
pub struct Post {
    #[index]
    pub post_id: u32,
    // uncomment this to test out cycle detection
    // #[owned_by(comments, text)]
    pub text: String,
    #[owned_by(users, user_id)]
    pub posted_by: u32,
    pub date: String,
}
```

## Examples
We provide 2 examples of using the `mongowner` library in the application code.

### social-rs
`social-rs` is a simple social media application that demonstrates the ease of use of our `mongowner` library. We provide a `Makefile` for easy building and running of the backend server. To run the backend for `social-rs`:
1) Ensure that you have an instance of mongod running - by default we assume that it is running on port `27017` but you can configure this by setting the `MONGO_URI` environment variable in the `.env` files located at `examples/social-rs/`
2) Once the database is running, you can run the following `make` command from the root of mongodog:
```
make run_server
```

To run the frontend for social-rs:
1) Ensure you have [bun](https://bun.sh/) installed
2) Run the following command to install all the dependencies:
```
make social_bun_deps
```
3) Lastly run the following to build and run the frontend client in developer mode:
```
make social_client
```

### websubmit-rs
This is a proof-of-concept application that highlights our library's ability to translate and model SQL ownership relations from [K9Db](https://github.com/brownsys/K9db/) to a NoSQL paradigm. We currently have a file named `mongo\_schemas.rs` that contain an example of what the annotated schemas may look like
Example of translating the `Answer` schema from K9Db's MySQL syntax to mongowner annotations:
Original K9Db (MySQL) Schema
```
CREATE TABLE answers (
    id varchar(255),
    email varchar(255),
    question_id int,
    answer text,
    submitted_at datetime,
    PRIMARY KEY (id),
    FOREIGN KEY (email) OWNED_BY users(email),
    FOREIGN KEY (question_id) REFERENCES questions(id)
);
```

`mongowner` schema
```
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
```

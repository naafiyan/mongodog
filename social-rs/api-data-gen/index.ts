import { faker } from "@faker-js/faker";
import * as readline from "readline";
import { exec } from "child_process";

type User = {
  user_id: string;
  username: string;
  first_name: string;
  last_name: string;
  age: number;
  email: string;
};

type Post = {
  text: string;
  posted_by: string;
  date: string;
};

const ENDPOINT_BASE: string = "http://127.0.0.1:8080";

export function createRandomUser(): User {
  return {
    user_id: faker.string.uuid(),
    username: faker.internet.userName(),
    first_name: faker.person.firstName(),
    last_name: faker.person.lastName(),
    age: faker.number.int({ min: 18, max: 100 }),
    email: faker.internet.email(),
  };
}

export function createRandomPost(posterId: string): Post {
  return {
    text: faker.word.words(),
    posted_by: posterId,
    date: faker.date.month(),
  };
}

async function populateDB() {
  const numUsers: number = 100;
  const numPosts: number = 1000;
  const users: User[] = [];
  const posts: Post[] = [];

  for (let i = 0; i < numUsers; i++) {
    users.push(createRandomUser());
  }

  for (let i = 0; i < numPosts; i++) {
    posts.push(createRandomPost(users[i % numUsers].user_id));
  }

  // make api request to endpoint (POST)

  const add_user_endpoint = `${ENDPOINT_BASE}/add_user`;
  console.log(add_user_endpoint);
  // send all the users
  console.log("Sending all the users now");
  for (let i = 0; i < numUsers; i++) {
    const response = await fetch(`${ENDPOINT_BASE}/add_user`, {
      method: "POST",
      body: JSON.stringify(users[i]),
      headers: {
        "Content-Type": "application/json",
      },
    });
  }
  console.log("Done sending users");
  //
  // const add_post_endpoint = `${ENDPOINT_BASE}/add_post`;
  // console.log(add_post_endpoint);
  // // send all the posts
  // console.log("Sending all the posts now");
  // for (let i = 0; i < numPosts; i++) {
  //   const response = await fetch(`${ENDPOINT_BASE}/add_post`, {
  //     method: "POST",
  //     body: JSON.stringify(posts[i]),
  //     headers: {
  //       "Content-Type": "application/json",
  //     },
  //   });
  //   const result = await response.json();
  //   console.log(result);
  // }
}

async function get_all_users() {
  const response = await fetch(`${ENDPOINT_BASE}/get_all_users`);
  const result = await response.json();
  console.log(result);
}

function main() {
  // REPL that reads user input and calls different functions based on input
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  // parse command line input arg db_name
  const db_name = process.argv[2];
  if (db_name === undefined) {
    console.log("Usage: bun run dev <db_name>");
    process.exit(1);
  }
  console.log(`Using database ${db_name}`);

  rl.question("Enter a command: ", async (answer) => {
    switch (answer) {
      case "populate":
        await populateDB();
        break;
      case "get_all_users":
        await get_all_users();
        break;
      case "delete_all_users":
        // call command line to delete all users
        exec(`mongosh --eval "use ${db_name}" --eval  "db.dropDatabase()"`);
        console.log("Deleted all users");
      case "quit":
        // quit the program
        process.exit(0);
      case "help":
        console.log(
          "Commands: populate, get_all_users, delete_all_users, quit"
        );
        break;
      default:
        console.log("Invalid command");
        break;
    }
    rl.close();
  });
}

main();

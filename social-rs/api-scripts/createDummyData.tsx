import { faker } from "@faker-js/faker";

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

async function main() {
  const endpoint: string = "127.0.0.1:8080";
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

  // send all the users
  console.log("Sending all the users now");
  for (let i = 0; i < numUsers; i++) {
    const response = await fetch(`${endpoint}/add_user`, {
      method: "POST",
      body: JSON.stringify(users[i]),
      headers: {
        "Content-Type": "application/json",
      },
    });
  }

  // send all the posts
  console.log("Sending all the posts now");
  for (let i = 0; i < numPosts; i++) {
    const response = await fetch(`${endpoint}/add_post`, {
      method: "POST",
      body: JSON.stringify(posts[i]),
      headers: {
        "Content-Type": "application/json",
      },
    });
    const result = response.json();
  }
}

main();

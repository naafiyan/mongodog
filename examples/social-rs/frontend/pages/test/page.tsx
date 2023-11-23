import React, {useEffect, useState} from 'react';
import axios from 'axios';
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
  } from "@/components/ui/card"

type User = {
  user_id: string;
  username: string;
  first_name: string;
  last_name: string;
  age: number;
  email: string;
};

type Post = {
  post_id: string;
  text: string;
  posted_by: string;
  date: string;
};

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [posts, setPosts] = useState<Post[]>([]);
    const [users, setUsers] = useState<User[]>([]);
    const [userDict, setUserDict] = useState({});
    // const [loading, setLoading] = useState(true);

    console.log(users);

    async function fetchPosts() {
        try {
          const response = await axios.get(`${ENDPOINT_BASE}/get_all_posts`, { 
          });
          setPosts(response.data);
        } catch (error) {
          console.error('Error fetching data:', error);
        }
      }
      
    async function fetchUsers() {
        try {
            const response = await axios.get(`${ENDPOINT_BASE}/get_all_users`, { 
            });
            const newUserDict = {};
            response.data.forEach((user: User) => {
                //@ts-ignore 
                newUserDict[user.user_id] = user.username;
            });
            setUserDict({...newUserDict});
            setUsers(response.data);
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    }

    function getUsername(user_id: string) {
        // @ts-ignore
        return userDict[user_id];
    }

    useEffect(() => {
        fetchUsers();
        fetchPosts();
        console.log({userDict})
    }, []);

    return <div className="p-4">
        <div className="flex flex-col gap-4 ">
    {posts.map((post) => (
            <Card key={post.post_id} className="w-64">
            <CardHeader>
                <CardTitle key={`${post.post_id}-${post.posted_by}`}> {getUsername(post.posted_by)}</CardTitle>
            </CardHeader>
            <CardContent>
                <CardDescription>
                {post.text}
                </CardDescription>
            </CardContent>
            <CardFooter>
                <CardDescription>
                {post.date}
                </CardDescription>
            </CardFooter>
            </Card>
    ))}
</div>
    </div>;
}

export default Page;
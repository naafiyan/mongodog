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
  text: string;
  posted_by: string;
  date: string;
};

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [posts, setPosts] = useState<Post[]>([]);
    // const [loading, setLoading] = useState(true);

    async function fetchPosts() {
        try {
          const response = await axios.get(`${ENDPOINT_BASE}/get_all_posts`, { 
          });
          setPosts(response.data);
        } catch (error) {
          console.error('Error fetching data:', error);
        }
      }
      

    useEffect(() => {
        fetchPosts();
    }, []);

    return <div>hello world
{posts.map((post) => (
        <Card key={post.text} className="w-64">
          <CardHeader>
            <CardTitle> {post.posted_by}</CardTitle>
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
    </div>;
}

export default Page;
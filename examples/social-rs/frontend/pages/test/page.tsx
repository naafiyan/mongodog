import React, {useEffect, useState} from 'react';
import axios from 'axios';

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
    // const [posts, setPosts] = useState<Post[]>([]);
    // const [loading, setLoading] = useState(true);

    async function fetchData() {
        try {
            //@ts-ignore
          const response = await axios.get(`${ENDPOINT_BASE}/get_all_posts`, { 
          });
          console.log(response.data);
        } catch (error) {
          console.error('Error fetching data:', error);
        }
      }
      

    useEffect(() => {
        fetchData();
    }, []);

    return <div>hello world
{/* 
        {posts.map((post: any) => <div>{post.user_id}
        {post.username}
        
        {post.last_name}
        </div>)} */}
    </div>;
}

export default Page;
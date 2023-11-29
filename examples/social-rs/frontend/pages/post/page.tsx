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
    import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    } from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { Button } from "@/components/ui/button"
import { useForm, SubmitHandler } from "react-hook-form"
import { useToast } from "@/components/ui/use-toast"
import dayjs from 'dayjs';



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

type Comment = {
    comment_id: string;
    commented_by: string;
    text: string;
    parent_post: string;
    date: string;
}

type Inputs = {
    text: string;
    posted_by: string;
  };

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [posts, setPosts] = useState<Post[]>([]);
    const [users, setUsers] = useState<User[]>([]);
    const [userDict, setUserDict] = useState({});
    const [colorDict, setColorDict] = useState({});

    const [comments, setComments] = useState<Comment[]>([]);
    const [currentUserId, setCurrentUserId] = useState<string>(""); 
    const {
        register,
        handleSubmit,
      } = useForm<Inputs>()
    const {toast} = useToast();

    async function fetchPosts() {
        try {
          const response = await axios.get(`${ENDPOINT_BASE}/get_all_posts`, { 
          });
          setPosts(response.data);
        } catch (error) {
          console.error('Error fetching data:', error);
        }
      }

      const colors = [
        '#9CF779',
        '#B888B3',
        '#F779EB',
        '#8EA286',
        '#786176'
    ]
      
    async function fetchUsers() {
        try {
            const response = await axios.get(`${ENDPOINT_BASE}/get_all_users`, { 
            });
            const newUserDict = {};
            const newColorDict = {};
            response.data.forEach((user: User, index: number) => {
                //@ts-ignore 
                newUserDict[user.user_id] = user.username;
                const color = colors[index % colors.length];
                //@ts-ignore
                newColorDict[user.user_id] = color;
            });
            setUserDict({...newUserDict});
            setColorDict({...newColorDict});
            setUsers(response.data);
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    }

         
    async function fetchComments() {
        try {
            const response = await axios.get(`${ENDPOINT_BASE}/get_all_comments`, { 
            });
            console.log(response.data)
            setComments(response.data);
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    }

    function getUsername(user_id: string) {
        // @ts-ignore
        return userDict[user_id];
    }

    const updateUsersAndPosts = () => {
        fetchUsers();
        fetchPosts();
        fetchComments();
    }

    useEffect(() => {
      updateUsersAndPosts();
    }, []);



    useEffect(() => {
        console.log({currentUserId})
    }, [currentUserId])

    const onSubmit: SubmitHandler<Inputs> = (data) => {
        console.log(data);
        axios.post(`${ENDPOINT_BASE}/add_post`, {
            text: data.text,
            posted_by: Number(currentUserId),
            date: new Date().toISOString(),
            post_id: Math.floor(Math.random()  * 100000)
        }).then((response) => {
            console.log(response);
            toast({
                title: response.statusText,
                description: response.data,
              })
        }).catch((error) => {
            console.error(error);
        })
    }

    function deletePost(post_id: string) {
        axios.delete(`${ENDPOINT_BASE}/delete_post/${post_id}`).then((response) => {
            console.log(response);
            toast({
                title: response.statusText,
                description: response.data,
              })
        }).catch((error) => {
            console.error(error);
        })
    }



    
    return <div className="p-4 flex flex-col gap-4">
        {/* <Button onClick={() => updateUsersAndPosts()}>Refresh Everything</Button> */}
        <form onSubmit={handleSubmit(onSubmit)}>
        <div className="flex flex-col gap-3">
        <Textarea {...register('text', {})} placeholder="Post something" />

        <div className="flex gap-3">
        <Select onValueChange={(value) => setCurrentUserId(value)}>
            <SelectTrigger>
            <SelectValue placeholder="Pick user" asChild>
                {getUsername(currentUserId)}
                </SelectValue>
            </SelectTrigger>
            <SelectContent>
                {users.map((user) => (
                    <SelectItem key={user.user_id} value={user.user_id}>{user.username}</SelectItem>
                ))}
            </SelectContent>
        </Select>
        <Button variant="outline" type="submit">Post</Button>
    </div>
        </div>
        </form>
        <div className="flex gap-4">
            
        <div className="flex flex-col gap-3 ">
            <h2>Posts</h2>
        {posts.sort((a,b) => dayjs(b.date).diff(dayjs(a.date))).map((post) => (
            // @ts-ignore
            <Card key={post.post_id} className="w-64" style={{backgroundColor: colorDict[post.posted_by]}}>
            <CardHeader>
                <div className="flex gap-3 justify-between w-full">
                <CardTitle key={`${post.post_id}-${post.posted_by}`}> {getUsername(post.posted_by)}</CardTitle>
                {post.post_id}
                <Button onClick={() => deletePost(post.post_id)}>Delete</Button>
                <div className="flex gap-4 justify-between w-half">
                    {comments.filter((comment) => comment.parent_post === post.post_id).map((comment) => (
                        // @ts-ignore
                        <div className="flex gap-5 border-solid border-2" style={{backgroundColor: colorDict[comment.commented_by]}}>
                            <div>{comment.text}</div>
                            <div>{getUsername(comment.commented_by)}</div>
                            </div>))}
                </div>
                </div>
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
</div>
    </div>;
}

export default Page;
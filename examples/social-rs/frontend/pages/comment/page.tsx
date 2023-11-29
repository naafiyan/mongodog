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
import { Button } from "@/components/ui/button"
import { useForm, SubmitHandler } from "react-hook-form"
import { Input } from "@/components/ui/input"
import dayjs from 'dayjs';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    } from "@/components/ui/select"
import { Textarea } from '@/components/ui/textarea';

type User = {
    user_id: string;
    username: string;
    first_name: string;
    last_name: string;
    age: number;
    email: string;
    };
      

type Comment = {
    comment_id: string;
    text: string;
    parent_post: string;
    date: string;
    commented_by: string;
}

type Inputs = {
    text: string;
    parent_post: string;
    date: string;
  };

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [comments, setComments] = useState<Comment[]>([]);
    const [users, setUsers] = useState<User[]>([]);
    const [userDict, setUserDict] = useState({});
    const [colorDict, setColorDict] = useState({});
    const [currentUserId, setCurrentUserId] = useState<string>(""); 
    const {
        register,
        handleSubmit,
      } = useForm<Inputs>()
    
      async function fetchComments() {
        try {
            const response = await axios.get(`${ENDPOINT_BASE}/get_all_comments`, { 
            });
            setComments(response.data);
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    }


    const colors = [
        '#3ABEFF',
        '#D84797',
        '#26FFE6',
        '#820933',
        '#D2FDFF'
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

    function getUsername(user_id: string) {
        // @ts-ignore
        return userDict[user_id];
    }


    useEffect(() => {
        fetchUsers();
      fetchComments();
    }, []);

    const onSubmit: SubmitHandler<Inputs> = (data) => {
        const {text, parent_post, date} = data;
        axios.post(`${ENDPOINT_BASE}/add_comment`, {
            text,
            parent_post: Number(parent_post),
            date: dayjs(date).format('YYYY-MM-DD'),
            comment_id: Math.floor(Math.random() * 100000),
            commented_by: Number(currentUserId)
        }).then((response) => {
            console.log(response);
        }).catch((error) => {
            console.error(error);
        })
    }


    function delete_comment(comment_id: string) {
        axios.delete(`${ENDPOINT_BASE}/delete_comment/${comment_id}`).then((response) => {
            console.log(response);
     
        }).catch((error) => {
            console.error(error);
        })
    }




    
    return <div className="p-4 flex flex-col gap-4">
              <form onSubmit={handleSubmit(onSubmit)}>
        <div className="flex flex-col gap-3 w-72">
        <Textarea {...register('text', {required: true})} placeholder="Comment text" />
        <Input {...register('parent_post', {required: true})} placeholder="Parent post ID" />
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
        <div className="flex gap-3">
       
        <Button variant="outline" type="submit">Post Comment</Button>
    </div>
        </div>
        </form>
        <div className="flex gap-4">

<div className="flex flex-col gap-3 ">
            <h2>Comments</h2>
            {/* @ts-ignore */}
        {comments.sort((a,b) => dayjs(b.date) - dayjs(a.date)).map((comment) => (
            // @ts-ignore
            <Card key={comment.comment_id} className="w-64" style={{backgroundColor: colorDict[comment.commented_by]}}>
            <CardHeader>
                <div className="flex gap-3 justify-between w-full">
                <CardTitle key={`${comment.text}`}>                     
                {comment.text}
            </CardTitle>
<Button onClick={() => delete_comment(comment.comment_id)}>Delete</Button>

                </div>
            </CardHeader>
            
            <CardContent>
                <CardDescription className="text-black">
                Parent Post: {comment.parent_post}
                </CardDescription>
            </CardContent>
            </Card>
    ))}
</div>
</div>
    </div>;
}

export default Page;
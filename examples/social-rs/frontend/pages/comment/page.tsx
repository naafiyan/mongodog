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




type Comment = {
    comment_id: string;
    text: string;
    parent_post: string;
    date: string;
}

type Inputs = {
    text: string;
    parent_post: string;
    date: string;
  };

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [comments, setComments] = useState<Comment[]>([]);
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


    useEffect(() => {
      fetchComments();
    }, []);

    const onSubmit: SubmitHandler<Inputs> = (data) => {
        const {text, parent_post, date} = data;
        axios.post(`${ENDPOINT_BASE}/add_comment`, {
            text,
            parent_post: Number(parent_post),
            date: dayjs(date).format('YYYY-MM-DD'),
            comment_id: Math.floor(Math.random() * 100000)
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
        <div className="flex flex-col gap-3">
        <Input {...register('text', {required: true})} placeholder="Comment text" />
        <Input {...register('parent_post', {required: true})} placeholder="Parent post ID" />
        <div className="flex gap-3">
       
        <Button variant="outline" type="submit">Post Comment</Button>
    </div>
        </div>
        </form>
        <div className="flex gap-4">

<div className="flex flex-col gap-3 ">
            <h2>Comments</h2>
        {comments.map((comment) => (
            <Card key={comment.comment_id} className="w-64">
            <CardHeader>
                <div className="flex gap-3 justify-between w-full">
                <CardTitle key={`${comment.text}`}>                     
                {comment.text}
            </CardTitle>
<Button onClick={() => delete_comment(comment.comment_id)}>Delete</Button>

                </div>
            </CardHeader>
            
            <CardContent>
                <CardDescription>
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
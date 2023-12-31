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

type Inputs = {
    username: string;
    first_name: string;
    last_name: string;
    age: number;
    email: string;
  };

const Page = () => {
    const ENDPOINT_BASE: string = "http://localhost:8080";
    const [users, setUsers] = useState<User[]>([]);
    const [userDict, setUserDict] = useState({});
    const [colorDict, setColorDict] = useState({});
    const {
        register,
        handleSubmit,
      } = useForm<Inputs>()
    

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
    }, []);

    const onSubmit: SubmitHandler<Inputs> = (data) => {
        const {username, first_name, last_name, email, age} = data;
        axios.post(`${ENDPOINT_BASE}/add_user`, {
            username,
            first_name,
            last_name,
            age: Number(age),
            email,
            user_id: Math.floor(Math.random() * 100000)
        }).then((response) => {
            console.log(response);
        }).catch((error) => {
            console.error(error);
        })
    }


    function delete_user(user_id: string) {
        axios.delete(`${ENDPOINT_BASE}/delete_user/${user_id}`).then((response) => {
            console.log(response);
     
        }).catch((error) => {
            console.error(error);
        })
    }




    
    return <div className="p-4 flex flex-col gap-4">
              <form onSubmit={handleSubmit(onSubmit)}>
        <div className="flex flex-col gap-3 w-72">
        <Input {...register('username', {required: true})} placeholder="Username" />
        <Input {...register('first_name', {required: true})} placeholder="First Name" />
        <Input {...register('last_name', {required: true})} placeholder="Last Name" />
        <Input {...register('email', {required: true})} placeholder="Email" />
        <Input {...register('age', {required: true})} placeholder="Age" />
        <div className="flex gap-3">
       
        <Button variant="outline" type="submit">Create User</Button>
    </div>
        </div>
        </form>
        <div className="flex gap-4">

<div className="flex flex-col gap-3 ">
            <h2>Users</h2>
        {users.map((user) => (
            // @ts-ignore
            <Card key={user.user_id} className="w-64" style={{backgroundColor: colorDict[user.user_id]}}>
            <CardHeader>
                <div className="flex gap-3 justify-between w-full">
                <CardTitle key={`${user.user_id}`}>                     {user.username}
</CardTitle>
<Button onClick={() => delete_user(user.user_id)}>Delete</Button>

                </div>
            </CardHeader>
            
            <CardContent>
                <CardDescription className="text-black">
                {user.email}
                </CardDescription>
            </CardContent>
            <CardFooter>
                <CardDescription className="text-black">
                {user.first_name} {user.last_name}
                </CardDescription>
            </CardFooter>
            </Card>
    ))}
</div>
</div>
    </div>;
}

export default Page;
"use client"

import { useAuth } from "./AuthContext";

const Profile = () => {
    const { username } = useAuth();

    return (
        <>  
            <p>Hello {username}</p>
        </>
    )
} 

export default Profile;

"use client"

import { jwtDecode } from "jwt-decode";
import { useState, useEffect } from "react";
import { useAuth } from "@/components/AuthContext";


const Profile = () => {
    const {user} = useAuth();
    return (
        <>
            <p>Hello {user?.username}</p>
        </>
    )
} 

export default Profile;

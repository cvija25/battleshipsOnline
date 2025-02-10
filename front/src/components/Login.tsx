"use client"

import axios from "axios";
import { useState } from "react";
import {jwtDecode} from "jwt-decode";

const Login = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [res, setRes] = useState("");
    const handleSubmit = async (e : React.FormEvent) => {
        e.preventDefault();

        const response = await axios.post("http://localhost:8000/login", {
            username: username,
            password: password
          }, {
            headers: {
              "Content-Type": "application/json",  // Required for JSON body
            }
          });

        const decoded = jwtDecode(response.data);
        setRes(decoded.username);
    }

    return (
        <>
            <form onSubmit={handleSubmit}>
                <input type="text" onChange={(e) => setUsername(e.target.value)}/>
                <input type="text" onChange={(e) => setPassword(e.target.value)}/>
                <button type="submit">try</button>
            </form>
            <p>{res}</p>
        </>
    )
}

export default Login;
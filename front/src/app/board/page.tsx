"use client"

import SetUp from "@/components/SetUpBoard";
import { useWebSocket } from "@/components/SocketContext";
import { useState, useEffect } from "react";

const Board = () => {
    const socket = useWebSocket();
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (!socket) return;

        socket.onmessage = (message:MessageEvent) => {
            console.log("Received message:", message);
            setLoading(false); // Mark loading as false on first message
        };

    }, [socket])
    return (
        <>
            <p>board</p>
            { loading ? (<div className="text-xl font-semibold">Loading...</div>) : (<SetUp rows={5} columns={5}/>)}
        </>
    );
}

export default Board;
"use client"

import SetUp from "@/components/SetUpBoard";
import { useWebSocket } from "@/components/SocketContext";
import { useState, useEffect } from "react";

const Board = () => {
    const ws = useWebSocket();
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (!ws) return;
    
        ws.onmessage = (event) => {
          console.log("Received:", event.data);
          setLoading(false);
        };
    
        return () => {
          ws.onmessage = null; // Clean up when component unmounts
        };
      }, [ws]);
    return (
        <>
            <p>board</p>
            { loading ? (<div className="text-xl font-semibold">Loading...</div>) : (<SetUp rows={5} columns={5}/>)}
        </>
    );
}

export default Board;
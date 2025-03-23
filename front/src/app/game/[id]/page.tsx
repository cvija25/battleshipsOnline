"use client"

import PlayerGrid from "@/components/PlayerGrid";
import { useWebSocket } from "@/components/SocketContext";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";

const GamePage = () => {
  const { id } = useParams(); 
  const ws = useWebSocket();
  const [loading, setLoading] = useState(true);
  const [play, setPlay] = useState(true);
  const [result,setResult] = useState("");
  const [end,setEnd] = useState(false);

  useEffect(() => {
    if (!ws) return;
    ws.send("ready");
    ws.onmessage = (event) => {
      console.log(event.data)
      if (event.data == "gameLoad") {
        setLoading(false);
      } else if (event.data == "ready") {
        setPlay(true);
      } else if (event.data == "no") {
        setPlay(false);
      } else if (event.data == "win" || event.data == "tie") {
        setResult(event.data);
        setEnd(true);
      } else {
        setResult(event.data);
      }
    };

    return () => {
      ws.onmessage = null; // Clean up when component unmounts
    };
  }, [ws]);
  return (
    <div>
      <main className="p-4">
        <h1 className="text-2xl font-bold mb-6">Game { id }</h1>
        {end ? (<div>GAME OVER</div>) : (
          <div>
            <h2>{result}</h2>
            { play ? (<div>your turn</div>) : (<div>not your turn</div>) }
            { loading ? (<div className="text-xl font-semibold">Loading...</div>) : (<PlayerGrid rows={5} columns={5} />)}
          </div>
        )}
      </main>
    </div>
  );
};

export default GamePage;

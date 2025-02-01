"use client"

import PlayerGrid from "@/components/PlayerGrid";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";

const GamePage = () => {
  const { id } = useParams(); 

  return (
    <div>
      <main className="p-4">
        <h1 className="text-2xl font-bold mb-6">Game { id }</h1>
        <PlayerGrid rows={5} columns={5} />
        <h2>{}</h2>
      </main>
    </div>
  );
};

export default GamePage;

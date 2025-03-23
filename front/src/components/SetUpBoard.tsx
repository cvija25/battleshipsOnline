"use client"

import Link from "next/link";
import { useWebSocket } from "./SocketContext";
import { useState } from "react";

interface GridProps {
    rows: number;
    columns: number;
    gameID: String;
}

interface Cell {
    row: number;
    col: number;
}

const SetUp:React.FC<GridProps> = ({ rows, columns, gameID }) => {
    const ws = useWebSocket();
    const handleClick = () => {
        ws?.send(JSON.stringify(clickedCells));
    }   
    const [clickedCells, setClickedCells] = useState<Cell[]>([]);
    const array = Array.from({length:rows});
    const handleCellClick = (row: number, col: number) => {
        const newClickedCells = [...clickedCells, { row, col }];
        setClickedCells(newClickedCells);
    };
    return (
        <>    
            <div>
                <div className="flex flex-col space-y-2">
                    {array.map((_,rowIndex) => (
                        <div key={rowIndex} className="flex space-x-2">
                        {array.map((_,colIndex) => (
                            <div key={colIndex}
                                className={`w-16 h-16 border border-gray-300 flex items-center justify-center text-sm cursor-pointer ${
                                clickedCells.some(
                                (cell) => cell.row === rowIndex && cell.col === colIndex
                                )
                                ? "bg-blue-300"
                                : "bg-white"
                            }`}
                            onClick={() => handleCellClick(rowIndex, colIndex)}
                            >
                            {`(${rowIndex}, ${colIndex})`}
                            </div>
                        ))}
                        </div>
                    ))}
                </div>
            </div>
            <Link href={'/game/'+gameID} onClick={handleClick}>
                <div className={`w-32 h-16 border border-gray-300 flex items-center justify-center text-sm cursor-pointer`}>Play</div>
            </Link>
        </>
    );
}

export default SetUp;
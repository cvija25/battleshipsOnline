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
            <div className="min-h-screen bg-blue-600 flex items-center justify-center p-4">
                <div className="flex flex-col space-y-4">
                    <div className="grid gap-2" style={{ gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))` }}>
                        {array.map((_,rowIndex) => (
                            array.map((_,colIndex) => (
                                <div key={`${rowIndex}-${colIndex}`}
                                    className={`w-16 h-16 bg-white rounded-lg border border-gray-200 flex items-center justify-center font-sans text-gray-800 text-sm cursor-pointer transition-colors duration-200 ${
                                        clickedCells.some(
                                            (cell) => cell.row === rowIndex && cell.col === colIndex
                                        )
                                        ? "bg-blue-300"
                                        : "bg-white hover:bg-gray-100"
                                    }`}
                                    onClick={() => handleCellClick(rowIndex, colIndex)}
                                >
                                    {`(${rowIndex}, ${colIndex})`}
                                </div>
                            ))
                        ))}
                    </div>
                    <Link href={'/game/'+gameID} onClick={handleClick}>
                        <div className="w-32 h-12 bg-blue-500 hover:bg-blue-600 text-white rounded-lg flex items-center justify-center font-sans text-base font-medium cursor-pointer transition-colors duration-200">
                            Play
                        </div>
                    </Link>
                </div>
            </div>
        </>
    );
}

export default SetUp;
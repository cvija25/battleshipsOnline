"use client"
import { useState } from "react";
import { useWebSocket } from "./SocketContext";
interface GridProps {
  rows: number;
  columns: number;
}

interface Cell {
  row: number;
  col: number;
}

const PlayerGrid: React.FC<GridProps> = ({ rows, columns }) => {
  const [clickedCells, setClickedCells] = useState<Cell[]>([]);
  const array = Array.from({length:rows});
  const socket = useWebSocket()
  const handleCellClick = (row: number, col: number) => {
    const newClickedCells = [...clickedCells, { row, col }];
    setClickedCells(newClickedCells);
    const to_send = {
      row:row,
      col:col
    }
    socket?.send(JSON.stringify(to_send));
  };
  return (
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
  );
};

export default PlayerGrid;

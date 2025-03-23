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
  const array = Array.from({length: rows});
  const socket = useWebSocket();

  const handleCellClick = (row: number, col: number) => {
    const newClickedCells = [...clickedCells, { row, col }];
    setClickedCells(newClickedCells);
    const to_send = {
      row: row,
      col: col
    };
    socket?.send(JSON.stringify(to_send));
  };

  return (
    <div className="min-h-screen bg-blue-600 flex items-center justify-center p-4">
      <div className="grid gap-2" style={{ gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))` }}>
        {array.map((_, rowIndex) => (
          array.map((_, colIndex) => (
            <div 
              key={`${rowIndex}-${colIndex}`}
              className={`w-16 h-16 bg-white rounded-lg border border-gray-200 flex items-center justify-center font-sans text-gray-800 text-sm cursor-pointer transition-colors duration-200 ${
                clickedCells.some(
                  (cell) => cell.row === rowIndex && cell.col === colIndex
                )
                  ? "bg-orange-500"
                  : "bg-white hover:bg-gray-100"
              }`}
              onClick={() => handleCellClick(rowIndex, colIndex)}
            >
              {`(${rowIndex}, ${colIndex})`}
            </div>
          ))
        ))}
      </div>
    </div>
  );
};

export default PlayerGrid;
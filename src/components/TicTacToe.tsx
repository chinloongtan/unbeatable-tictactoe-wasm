"use client"

import React, { useState, useEffect, useRef } from 'react';
import { initWasm } from '@/utils/wasmLoader';

const TicTacToe = () => {
  const [board, setBoard] = useState(Array(9).fill(null)); // Initialize an empty 3x3 board
  const [isXNext, setIsXNext] = useState(true); // Track whose turn it is
  const [winner, setWinner] = useState(null); // Track the winner
  const [computerFirst, setComputerFirst] = useState(false);
  const wasm = useRef<any>(null);
  const [isDraw, setIsDraw] = useState(false);

  useEffect(() => {
    (async () => {
      wasm.current = await initWasm();
    })();
  }, []);

  // Winning combinations on the board
  const winningCombinations = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
  ];

  // Check if there's a winner
  const checkWinner = (board) => {
    const stepCount = board.filter((move) => Boolean(move)).length;
    if (stepCount >= 9) {
      setIsDraw(true);
      return null;
    }
    for (const combination of winningCombinations) {
      const [a, b, c] = combination;
      if (board[a] && board[a] === board[b] && board[a] === board[c]) {
        return board[a];
      }
    }
    return null;
  };

  // Handle click on a cell
  const handleClick = (index) => {
    if (board[index] || winner) return; // Ignore if the cell is already filled or game is won
    const newBoard = [...board];
    newBoard[index] = isXNext ? 'X' : 'O';
    setBoard(newBoard);
    setIsXNext(!isXNext);
    const gameWinner = checkWinner(newBoard);
    setWinner(gameWinner);
  };

  useEffect(() => {
    const isComputerNext = (computerFirst && isXNext) || (!computerFirst && !isXNext);
    if (isComputerNext) { // isXNext is current move
        play();
    }
  }, [isXNext, computerFirst]);


  const play = () => {
    const xIndices = board
      .map((value, index) => value === 'X' ? index + 1 : -1) // Map 'X' to its index, others to -1
      .filter(index => index !== -1); // Remove -1 values

    const oIndices = board
      .map((value, index) => value === 'O' ? index + 1 : -1) // Map 'O' to its index, others to -1
      .filter(index => index !== -1); // Remove -1 values

      const params = {
        perfect_player: computerFirst ? xIndices : oIndices,
        opponent: computerFirst ? oIndices : xIndices,
        first_hand: computerFirst,
      }

      const nextStep = wasm.current.play(params.perfect_player, params.opponent, params.first_hand)
      if (nextStep > 0) {
        handleClick(nextStep - 1);
      } else if (nextStep === -1) {
        setIsDraw(true);
      }
  }

  // Restart the game
  const restartGame = () => {
    setBoard(Array(9).fill(null));
    setIsXNext(true);
    setWinner(null);
    setIsDraw(false);
  };

  const playAs = (xo: 'X' | 'O') => {
    if (xo === 'O') {
      setComputerFirst(true);
    } else {
      setComputerFirst(false);
    }
    restartGame();
  }

  const showResult = () => {
    if (isDraw) {
      return "It's a draw";
    }

    if ((winner === 'X' && computerFirst) || (winner === 'O' && !computerFirst)) {
      return 'You lose';
    }

    if ((winner === 'X' && !computerFirst) || (winner === 'O' && computerFirst)) {
      return 'You win';
    }
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-800 text-white p-4">

      <div className="flex mb-8">
        <button
          className={`px-6 py-4 bg-red-500 text-white rounded-lg font-black text-xl hover:bg-red-700  mr-1 ${computerFirst && 'opacity-30'}`}
          onClick={() => {
            playAs('X');
          }}
        >
          X
        </button>

        <button
          className={`px-6 py-4 bg-green-500 text-white rounded-lg font-black text-xl hover:bg-green-700 ml-1 ${!computerFirst && 'opacity-30'}`}
          onClick={() => {
            playAs('O');
          }}
        >
          O
        </button>
      </div>


      {/* Tic-Tac-Toe Board */}
      <div className="grid grid-cols-3 gap-2 mb-4">
        {board.map((cell, index) => (
          <button
            key={index}
            className="w-20 h-20 flex items-center justify-center text-2xl font-bold rounded-lg border-2 border-gray-600 bg-gray-700 hover:bg-gray-600"
            onClick={() => handleClick(index)}
            style={{
              color: cell === 'X' ? 'red' : 'green'
            }}
          >
            {cell}
          </button>
        ))}
      </div>

      <div className="h-12">
        {(Boolean(winner) || isDraw) && (
          <div className="text-xl font-semibold mb-4 text-white">{showResult()}</div>
        )}
      </div>

      <div className="h-12">
        {
          (Boolean(winner) || isDraw) && (
            <button
              className={`h-12 px-6 py-4 text-blue-300 rounded-lg font-semibold text-md hover:text-blue-500`}
              onClick={() => {
                playAs(computerFirst ? 'O' : 'X')
              }}
            >
              Play again
            </button>
          )
        }
      </div>
    </div>
  );
};

export default TicTacToe;

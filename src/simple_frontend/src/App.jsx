import React, { useState } from 'react';
import Game from './components/Game';
import MintNFT from './components/MintNFT';

function App() {
  const [score, setScore] = useState(0);

  return (
    <div className="App">
      <h1>Game & Mint NFT</h1>
      <Game setScore={setScore} />
      <MintNFT score={score} />
    </div>
  );
}

export default App;

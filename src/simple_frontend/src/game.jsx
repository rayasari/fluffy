import React, { useState } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from 'dfx-generated/backend';

const agent = new HttpAgent();
const backend = Actor.createActor(idlFactory, { agent, canisterId: "your_canister_id" });

function Game({ setScore }) {
  const [localScore, setLocalScore] = useState(0);

  const increaseScore = () => {
    const newScore = localScore + 10;
    setLocalScore(newScore);
    setScore(newScore);
    backend.submit_score(agent.getPrincipal(), newScore);
  };

  return (
    <div>
      <p>Your Score: {localScore}</p>
      <button onClick={increaseScore}>Increase Score</button>
    </div>
  );
}

export default Game;

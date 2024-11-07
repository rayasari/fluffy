import React, { useState } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from 'dfx-generated/backend';

const agent = new HttpAgent();
const backend = Actor.createActor(idlFactory, { agent, canisterId: "your_canister_id" });

function MintNFT({ score }) {
  const [metadata, setMetadata] = useState('');
  const [nftId, setNftId] = useState(null);

  const mintNFT = async () => {
    if (score >= 50) { // Minimum score requirement to mint NFT
      const id = await backend.mint_nft(metadata, agent.getPrincipal());
      setNftId(id);
    } else {
      alert('Score too low to mint NFT');
    }
  };

  return (
    <div>
      <input
        type="text"
        placeholder="NFT Metadata URL"
        value={metadata}
        onChange={(e) => setMetadata(e.target.value)}
      />
      <button onClick={mintNFT}>Mint NFT</button>
      {nftId && <p>Minted NFT ID: {nftId}</p>}
    </div>
  );
}

export default MintNFT;

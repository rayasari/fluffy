use ic_cdk::export::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct NFT {
    id: u64,
    owner: Principal,
    metadata: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct PuzzleGame {
    score: u64,
}

#[ic_cdk_macros::init]
fn init() {}

static mut NFTS: Option<HashMap<u64, NFT>> = None;
static mut GAMES: Option<HashMap<Principal, PuzzleGame>> = None;
static mut NEXT_ID: u64 = 1;

#[ic_cdk_macros::update]
fn mint_nft(metadata: String, owner: Principal) -> u64 {
    let nft_id;
    unsafe {
        let nfts = NFTS.get_or_insert(HashMap::new());
        nft_id = NEXT_ID;
        NEXT_ID += 1;
        nfts.insert(nft_id, NFT { id: nft_id, owner, metadata });
    }
    nft_id
}

#[ic_cdk_macros::update]
fn submit_score(player: Principal, score: u64) {
    unsafe {
        let games = GAMES.get_or_insert(HashMap::new());
        games.insert(player, PuzzleGame { score });
    }
}

#[ic_cdk_macros::query]
fn get_nfts(owner: Principal) -> Vec<NFT> {
    unsafe {
        NFTS
            .as_ref()
            .unwrap_or(&HashMap::new())
            .values()
            .filter(|nft| nft.owner == owner)
            .cloned()
            .collect()
    }
}

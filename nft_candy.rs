extern crate soroban_sdk;
use soroban_sdk::prelude::*;

// Define the structure of an NFT
struct NFT {
    owner: PublicKey,
    metadata: String, // You can include metadata for your NFTs
}

// Define the NFT Candy Machine contract
#[contract]
pub fn NFTCandyMachine(
    #[state]
    nfts: StorageValue<Vec<NFT>>,
) {
    // Function to mint an NFT
    fn mint_nft(
        owner: PublicKey,
        metadata: String,
    ) {
        let nft = NFT { owner, metadata };
        nfts.push(nft);
    }
}

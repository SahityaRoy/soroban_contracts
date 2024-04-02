extern crate soroban_sdk;
use soroban_sdk::prelude::*;

// Define the structure of a non-fungible token
struct NFT {
    owner: PublicKey,
    token_id: u64,
}

impl NFT {
    fn new(owner: PublicKey, token_id: u64) -> Self {
        Self { owner, token_id }
    }
}

// Define the marketplace contract
#[contract]
pub fn NFTMarketplace(
    #[state]
    nfts: StorageValue<Vec<NFT>>,
) {
    // Function to create a new NFT
    #[constructor]
    fn create_nft(
        owner: PublicKey,
        token_id: u64,
    ) {
        let nft = NFT::new(owner, token_id);
        nfts.push(nft);
    }

    // Function to buy an NFT
    fn buy_nft(
        buyer: PublicKey,
        token_id: u64,
    ) {
        let mut nft_index = None;
        for (i, nft) in nfts.iter().enumerate() {
            if nft.token_id == token_id {
                nft_index = Some(i);
                break;
            }
        }

        if let Some(index) = nft_index {
            // Transfer ownership
            let mut nft = nfts.remove(index);
            nft.owner = buyer;
            nfts.push(nft);
        } else {
            panic!("NFT not found");
        }
    }

    // Function to transfer ownership of an NFT
    fn transfer_nft(
        new_owner: PublicKey,
        token_id: u64,
    ) {
        let mut nft_index = None;
        for (i, nft) in nfts.iter().enumerate() {
            if nft.token_id == token_id {
                nft_index = Some(i);
                break;
            }
        }

        if let Some(index) = nft_index {
            // Update ownership
            nfts[index].owner = new_owner;
        } else {
            panic!("NFT not found");
        }
    }
}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ChunkCreatedEvent {
    pub ticket_id: i64,
    pub chunk_index: i64,
    pub timestamp: i64,
}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct JackpotClaimEvent {
    pub ticket_id: i64,
    pub winner: Pubkey,
    pub amount: i64,
    pub chunk_index: i64,
    pub index_in_chunk: i64,
    pub timestamp: i64,
}
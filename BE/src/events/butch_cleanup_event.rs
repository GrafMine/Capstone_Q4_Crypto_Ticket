use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct BatchCleanupEvent {
    pub ticket_id: i64,
    pub chunks_cleaned: i64,
    pub timestamp: i64,
}

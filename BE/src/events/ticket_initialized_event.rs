use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TicketInitializedEvent {
    pub ticket_id: i64,
    pub admin: Pubkey,
    pub price: i64,
    pub timestamp: i64,
}
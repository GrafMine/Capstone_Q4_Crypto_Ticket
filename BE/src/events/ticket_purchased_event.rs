use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TicketPurchasedEvent {
    pub ticket_id: i64,
    pub amount: i64,
    pub chunk_index: i64,
    pub participant_index: i64,
    pub timestamp: i64,

    pub user_id: Pubkey,
    pub user_field: [i8; 9],

    pub round_index: i8,
    pub round_dir: i8,
    pub round_diff: i8,
}

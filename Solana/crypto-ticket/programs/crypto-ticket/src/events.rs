//programs/crypto-ticket/src/events.rs
use anchor_lang::prelude::*;

// События
#[event]
pub struct TicketInitializedEvent {
    pub ticket_id: i64,
    pub admin: Pubkey,
    pub price: i64,
    pub timestamp: i64,
}

#[event]
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

#[event]
pub struct JackpotClaimEvent {
    pub ticket_id: i64,
    pub winner: Pubkey,
    pub amount: i64,
    pub chunk_index: i64,
    pub index_in_chunk: i64,
    pub timestamp: i64,
}

#[event]
pub struct ChunkCreatedEvent {
    pub ticket_id: i64,
    pub chunk_index: i64,
    pub timestamp: i64,
}

#[event]
pub struct ChunkCleanedEvent {
    pub ticket_id: i64,
    pub chunk_index: i64,
    pub timestamp: i64,
}

#[event]
pub struct BatchCleanupEvent {
    pub ticket_id: i64,
    pub chunks_cleaned: i64,
    pub timestamp: i64,
}

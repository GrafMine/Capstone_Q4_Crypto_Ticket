//programs/crypto-ticket/src/events.rs
use anchor_lang::prelude::*;

// События
#[event]
pub struct TicketInitializedEvent {
    pub ticket_id: u64,
    pub admin: Pubkey,
    pub price: u64,
    pub timestamp: u64,
}

#[event]
pub struct TicketPurchasedEvent {
    pub ticket_id: u64,
    pub amount: u64,
    pub chunk_index: u64,
    pub participant_index: u64,
    pub timestamp: u64,

    pub user_id: Pubkey,
    pub user_field: [u8; 9],

    pub round_index: u8,
    pub round_dir: u8,
    pub round_diff: u8,
}

#[event]
pub struct JackpotClaimEvent {
    pub ticket_id: u64,
    pub winner: Pubkey,
    pub amount: u64,
    pub chunk_index: u64,
    pub index_in_chunk: u64,
    pub timestamp: u64,
}

#[event]
pub struct TicketFinishedEvent {
    pub ticket_id: u64,
    pub total_participants: u64,
    pub total_jackpot: u64,
    pub timestamp: u64,
}

#[event]
pub struct ChunkCreatedEvent {
    pub ticket_id: u64,
    pub chunk_index: u64,
    pub timestamp: u64,
}

#[event]
pub struct ChunkCleanedEvent {
    pub ticket_id: u64,
    pub chunk_index: u64,
    pub timestamp: u64,
}

#[event]
pub struct BatchCleanupEvent {
    pub ticket_id: u64,
    pub chunks_cleaned: u64,
    pub timestamp: u64,
}

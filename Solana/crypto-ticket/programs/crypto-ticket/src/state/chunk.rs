//programs/crypto-ticket/src/state/chunk.rs
use anchor_lang::prelude::*;

#[account]
pub struct ParticipantsChunk {
    pub ticket_id: u64,
    pub chunk_index: u64,
    pub current_count: u64,
    pub participants: Vec<Pubkey>,
}


impl ParticipantsChunk {
    pub const CHUNK_SIZE: usize = 1000;

    pub fn space() -> usize {
        8 + // discriminator
        8 + // ticket_id
        8 + // chunk_index
        8 + // current_count
        4 + // Vec length
        (32 * Self::CHUNK_SIZE) // Space for 1000 pubkeys
    }
}

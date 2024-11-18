//programs/crypto-ticket/src/state/chunk.rs
use anchor_lang::prelude::*;

use super::Player;
use super::Round;

#[account]
pub struct ParticipantsChunk {
    pub ticket_id: i64,
    pub chunk_index: i64,
    pub current_count: i64,
    pub participants: Vec<Player>,
    pub rounds_history: Vec<Round>,
}

impl ParticipantsChunk {
    pub const CHUNK_SIZE: usize = 50;

    pub fn space() -> usize {
        8 + // discriminator
        8 + // ticket_id
        8 + // chunk_index
        8 + // current_count
        (Player::LEN * Self::CHUNK_SIZE) + // Space for 300 pubkeys
        (Round::LEN * Self::CHUNK_SIZE) // Space for 300 pubkeys
    }
}

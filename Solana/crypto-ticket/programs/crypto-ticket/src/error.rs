//programs/crypto-ticket/src/error.rs
use anchor_lang::prelude::*;
// Ошибки
#[error_code]
pub enum ErrorCode {
    #[msg("Ticket is not active")]
    TicketNotActive,

    #[msg("Unauthorized access")]
    UnauthorizedAccess,

    #[msg("Invalid ticket jackpot combination")]
    InvalidTicketJackpot,

    #[msg("Jackpot already claimed")]
    JackpotAlreadyClaimed,

    #[msg("No participants in the lottery")]
    NoParticipants,

    #[msg("Current chunk is full, need to create new one")]
    ChunkIsFull,

    #[msg("Invalid chunk index")]
    InvalidChunkIndex,

    #[msg("Invalid chunk address")]
    InvalidChunkAddress,

    #[msg("Ticket is still active")]
    TicketStillActive,

    #[msg("Invalid winner")]
    InvalidWinner,
}

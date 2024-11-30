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

    #[msg("Empty jackpot")]
    EmptyJackpot,

    #[msg("Invalid jackpot amount")]
    InvalidJackpotAmount,
    
    #[msg("No participants in the lottery")]
    NoParticipants,

    #[msg("Current chunk is full, need to create new one")]
    ChunkIsFull,

    #[msg("Invalid chunk index")]
    InvalidChunkIndex,

    #[msg("Invalid chunk address")]
    InvalidChunkAddress,

    #[msg("Invalid chunk data")]
    InvalidChunkData,

    #[msg("Participant count mismatch")]
    ParticipantCountMismatch,

    #[msg("Missing chunks")]
    MissingChunks,

    #[msg("Invalid chunk sequence")]
    InvalidChunkSequence,

    #[msg("Invalid timestamp sequence")]
    InvalidTimestampSequence,

    #[msg("Ticket is still active")]
    TicketStillActive,

    #[msg("Invalid winner")]
    InvalidWinner,

    #[msg("Winner not verified")]
    WinnerNotVerified,

    #[msg("Invalid randomness data")]
    InvalidRandomnessData,

    #[msg("Randomness not yet resolved")]
    RandomnessNotResolved,

    #[msg("Calculation error")]
    CalculationError,
}

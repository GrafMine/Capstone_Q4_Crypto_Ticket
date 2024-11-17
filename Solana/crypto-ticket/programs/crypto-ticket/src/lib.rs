//programs/crypto-ticket/src/lib.rs
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::{
    initialize::*,
    // cleanup::*,
    // ticket::*,
    // claim::*
};

pub use state::{
    chunk::ParticipantsChunk,
    ticket::{TicketAccount, TicketJackpot}
};

declare_id!("EMGePWFB5TKj6hDYX1q6tfB6PJNhRLigBVtS7ZNyW5zf");

#[program]
pub mod crypto_ticket {
    use super::*;

    // // Базовая инициализация
        pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
            // initialize_handler(ctx)
             handler(ctx)
        }

        // Инициализация билета
        // pub fn init_ticket(
        //     ctx: Context<InitializeTicket>,
        //     ticket_id: u64,
        //     price: u64,
        // ) -> Result<()> {
        //     initialize_ticket(ctx, ticket_id, price)
        // }

        // Инициализация чанка участников
        // pub fn init_participants_chunk(
        //     ctx: Context<InitializeParticipantsChunk>,
        //     ticket_id: u64,
        //     chunk_index: u64,
        // ) -> Result<()> {
        //     initialize_participants_chunk(ctx, ticket_id, chunk_index)
        // }
}

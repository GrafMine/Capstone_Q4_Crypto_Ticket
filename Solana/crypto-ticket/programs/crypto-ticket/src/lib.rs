//programs/crypto-ticket/src/lib.rs
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::{
    initialize::*,
    finish::*,
    claim::*,
    // cleanup::*,
    ticket::*,
    // claim::*
};

pub use state::{
    chunk::ParticipantsChunk,
    ticket::{TicketAccount, TicketJackpot}
};

declare_id!("8sKVvV5NTamS36qakrS7qm45W2xxgmXPMrmGn4NH2gsm");

#[program]
pub mod crypto_ticket {

    use super::*;

        pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
             handler(ctx)
        }

        // Инициализация билета
        pub fn init_ticket(
            ctx: Context<InitializeTicket>,
            ticket_id: u64,
            price: u64,
        ) -> Result<()> {
            initialize_ticket(ctx, ticket_id, price)
        }

        // Завершение билета
        pub fn finish_ticket(
            ctx: Context<FinishTicket>,
            ticket_id: u64,
        ) -> Result<()> {
            fin_ticket(ctx, ticket_id)
        }

        // Выплатить джекпот
        pub fn claim_jackpot_handler<'info>(
            ctx: Context<'_, '_, 'info, 'info, ClaimJackpot<'info>>,
            ticket_id: u64,
        ) -> Result<()> {
            claim_jackpot(ctx, ticket_id)
        }

        // Инициализация чанка участников
        // pub fn init_participants_chunk(
        //     ctx: Context<InitializeParticipantsChunk>,
        //     ticket_id: u64,
        //     chunk_index: u64,
        // ) -> Result<()> {
        //     initialize_participants_chunk(ctx, ticket_id, chunk_index)
        // }

        pub fn buy(
            ctx: Context<BuyTicket>,
            ticket_id: u64
        ) -> Result<()> {
            buy_ticket(ctx, ticket_id)
        }
}

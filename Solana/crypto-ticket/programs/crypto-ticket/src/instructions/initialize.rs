//programs/crypto-ticket/src/instructions/initialize.rs
use crate::state::{ParticipantsChunk, TicketAccount, TicketJackpot};
use crate::events::{TicketInitializedEvent, ChunkCreatedEvent};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}

// Создание нового билета администратором
pub fn initialize_ticket(
    ctx: Context<InitializeTicket>,
    ticket_id: i64,
    price: i64,
) -> Result<()> {
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;
    let first_chunk = &mut ctx.accounts.first_participants_chunk;

    // Инициализация билета
    ticket_account.is_active = true;
    ticket_account.admin = ctx.accounts.admin.key();
    ticket_account.ticket_id = ticket_id;
    ticket_account.price = price;
    ticket_account.total_participants = 0;

    // Инициализация джекпота для этого билета
    ticket_jackpot.total_amount = 0;
    ticket_jackpot.winner = Pubkey::default();
    ticket_jackpot.ticket_id = ticket_id;
    ticket_jackpot.is_claimed = false;

    // Инициализация первого чанка участников
    first_chunk.ticket_id = ticket_id;
    first_chunk.chunk_index = 0;
    first_chunk.current_count = 0;
    first_chunk.participants = Vec::with_capacity(ParticipantsChunk::CHUNK_SIZE);

    emit!(TicketInitializedEvent {
        ticket_id,
        admin: ctx.accounts.admin.key(),
        price,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// Создание нового чанка участников
pub fn initialize_participants_chunk(
    ctx: Context<InitializeParticipantsChunk>,
    ticket_id: i64,
    chunk_index: i64,
) -> Result<()> {
    let chunk = &mut ctx.accounts.participants_chunk;
    chunk.ticket_id = ticket_id;
    chunk.chunk_index = chunk_index;
    chunk.current_count = 0;
    chunk.participants = Vec::with_capacity(ParticipantsChunk::CHUNK_SIZE);

    emit!(ChunkCreatedEvent {
        ticket_id,
        chunk_index,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(ticket_id: i64)]  // Добавляем ticket_id как параметр
pub struct InitializeTicket<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 1 + 32 + 8 + 8 + 8,
        seeds = [b"ticket", ticket_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 32 + 8 + 1,
        seeds = [b"jackpot", ticket_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_jackpot: Account<'info, TicketJackpot>,

    #[account(
        init,
        payer = admin,
        space = ParticipantsChunk::space(),
        seeds = [b"participants", ticket_id.to_le_bytes().as_ref(), 0u64.to_le_bytes().as_ref()],
        bump
    )]
    pub first_participants_chunk: Account<'info, ParticipantsChunk>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ticket_id: u64, chunk_index: u64)]  // Добавляем параметры
pub struct InitializeParticipantsChunk<'info> {
    #[account(
        init,
        payer = payer,
        space = ParticipantsChunk::space(),
        seeds = [
            b"participants",
            ticket_id.to_le_bytes().as_ref(),
            chunk_index.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub participants_chunk: Account<'info, ParticipantsChunk>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

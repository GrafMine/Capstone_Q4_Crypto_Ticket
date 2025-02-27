//programs/crypto-ticket/src/instructions/cleanup.rs
use crate::events::{ChunkCleanedEvent, BatchCleanupEvent};
use crate::log_event;
use crate::state::{ParticipantsChunk, TicketAccount};
use anchor_lang::prelude::*;
use crate::error::ErrorCode;

// Очистка чанка участников
pub fn cleanup_participants_chunk(ctx: Context<CleanupParticipantsChunk>) -> Result<()> {
    let chunk = &ctx.accounts.participants_chunk;
    let admin = &ctx.accounts.admin;

    require!(
        !ctx.accounts.ticket_account.is_active,
        ErrorCode::TicketStillActive
    );

    // Возвращаем rent exempt lamports админу
    let chunk_lamports = chunk.to_account_info().lamports();
    **chunk.to_account_info().try_borrow_mut_lamports()? = 0;
    **admin.to_account_info().try_borrow_mut_lamports()? += chunk_lamports;

    log_event!(ChunkCleanedEvent {
        ticket_id: chunk.ticket_id,
        chunk_index: chunk.chunk_index,
        timestamp: Clock::get()?.unix_timestamp as u64,
    });

    Ok(())
}

// Пакетная очистка чанков
pub fn batch_cleanup_chunks(ctx: Context<BatchCleanupChunks>, chunk_indexes: Vec<u64>) -> Result<()> {
    let ticket_account = &ctx.accounts.ticket_account;
    require!(!ticket_account.is_active, ErrorCode::TicketStillActive);

    for (i, chunk_account) in ctx.remaining_accounts.iter().enumerate() {
        let ticket_bytes = ticket_account.ticket_id.to_le_bytes();
        let chunk_bytes = chunk_indexes[i].to_le_bytes();

        let chunk_seeds = &[
            b"participants",
            ticket_bytes.as_ref(),
            chunk_bytes.as_ref(),
        ];

        let (expected_address, _) = Pubkey::find_program_address(chunk_seeds, ctx.program_id);
        require!(chunk_account.key() == expected_address, ErrorCode::InvalidChunkAddress);

        let chunk_lamports = chunk_account.lamports();
        **chunk_account.try_borrow_mut_lamports()? = 0;
        **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += chunk_lamports;
    }

    log_event!(BatchCleanupEvent {
        ticket_id: ticket_account.ticket_id,
        chunks_cleaned: chunk_indexes.len() as u64,
        timestamp: Clock::get()?.unix_timestamp as u64,
    });

    Ok(())
}


#[derive(Accounts)]
pub struct CleanupParticipantsChunk<'info> {
    #[account(
        mut,
        close = admin,
        constraint = !ticket_account.is_active
    )]
    pub participants_chunk: Account<'info, ParticipantsChunk>,

    pub ticket_account: Account<'info, TicketAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchCleanupChunks<'info> {
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

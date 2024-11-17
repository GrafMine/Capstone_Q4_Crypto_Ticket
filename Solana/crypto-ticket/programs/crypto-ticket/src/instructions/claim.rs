use crate::state::{ParticipantsChunk, TicketAccount, TicketJackpot, TicketHistory};
use switchboard_v2::AggregatorAccountData;
use crate::events::JackpotClaimEvent;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

// Выплата джекпота победителю
pub fn claim_jackpot(ctx: Context<ClaimJackpot>, ticket_id: u64) -> Result<()> {
    let ticket_account = &ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;

    // Проверки
    require!(
        ticket_account.admin == ctx.accounts.admin.key(),
        ErrorCode::UnauthorizedAccess
    );
    require!(
        ticket_account.ticket_id == ticket_jackpot.ticket_id,
        ErrorCode::InvalidTicketJackpot
    );
    require!(!ticket_jackpot.is_claimed, ErrorCode::JackpotAlreadyClaimed);
    require!(ticket_account.total_participants > 0, ErrorCode::NoParticipants);

    // Получаем случайное число от оракула
    let feed = &ctx.accounts.feed_aggregator.load()?;
    let random_value: u64 = feed.get_result()?.try_into()?;

    // Определяем победителя
    let winner_index = random_value % ticket_account.total_participants;
    let chunk_index = winner_index / ParticipantsChunk::CHUNK_SIZE as u64;
    let index_in_chunk = winner_index % ParticipantsChunk::CHUNK_SIZE as u64;

    // Проверяем чанк победителя
    let participants_chunk = &ctx.accounts.winner_participants_chunk;
    require!(
        participants_chunk.chunk_index == chunk_index,
        ErrorCode::InvalidChunkIndex
    );

    // Получаем победителя
    let winner_pubkey = participants_chunk.participants[index_in_chunk as usize];
    require!(
        ctx.accounts.winner.key() == winner_pubkey,
        ErrorCode::InvalidWinner
    );

    // Выплата джекпота
    let amount = ticket_jackpot.total_amount;
    **ticket_jackpot.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += amount;

    // Сохраняем историю
    let history = &mut ctx.accounts.ticket_history;
    history.ticket_id = ticket_id;
    history.winner = winner_pubkey;
    history.amount = amount;
    history.participants_count = ticket_account.total_participants;
    history.claimed_at = Clock::get()?.unix_timestamp;

    // Обновляем состояние
    ticket_jackpot.is_claimed = true;
    ticket_jackpot.winner = winner_pubkey;
    ticket_account.is_active = false;

    emit!(JackpotClaimEvent {
        ticket_id,
        winner: winner_pubkey,
        amount,
        chunk_index,
        index_in_chunk,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimJackpot<'info> {
    #[account(mut)]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(mut)]
    pub ticket_jackpot: Account<'info, TicketJackpot>,

    #[account(
        seeds = [
            b"participants",
            ticket_account.ticket_id.to_le_bytes().as_ref(),
            winner_chunk_index.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub winner_participants_chunk: Account<'info, ParticipantsChunk>,

    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 32 + 8 + 8 + 8, // discriminator + ticket_id + pubkey + amount + participants_count + timestamp
        seeds = [b"history", ticket_account.ticket_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_history: Account<'info, TicketHistory>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: Winner receives jackpot
    #[account(mut)]
    pub winner: AccountInfo<'info>,

    /// CHECK: Switchboard aggregator account
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,

    pub system_program: Program<'info, System>,
}

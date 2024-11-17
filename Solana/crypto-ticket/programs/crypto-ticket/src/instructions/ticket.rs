//programs/crypto-ticket/src/instructions/ticket.rs
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::state::{ParticipantsChunk, TicketAccount, TicketJackpot};
use crate::events::TicketPurchasedEvent;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(mut)]
    pub ticket_jackpot: Account<'info, TicketJackpot>,

    #[account(mut)]
    pub current_participants_chunk: Account<'info, ParticipantsChunk>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Admin receives fees
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


// Покупка билета пользователем
pub fn buy_ticket(ctx: Context<BuyTicket>, ticket_id: u64) -> Result<()> {
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;
    let current_chunk = &mut ctx.accounts.current_participants_chunk;

    // Проверки
    require!(ticket_account.is_active, ErrorCode::TicketNotActive);
    require!(!ticket_jackpot.is_claimed, ErrorCode::JackpotAlreadyClaimed);
    require!(ticket_account.ticket_id == ticket_jackpot.ticket_id, ErrorCode::InvalidTicketJackpot);

    // Проверяем, не заполнен ли текущий чанк
    if current_chunk.current_count >= ParticipantsChunk::CHUNK_SIZE as u64 {
        return Err(ErrorCode::ChunkIsFull.into());
    }

    // Рассчитываем комиссию (10%)
    let amount = ticket_account.price;
    let fee = amount / 10;
    let jackpot_amount = amount - fee;

    // Переводим комиссию админу
    invoke(
        &system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.admin.key(),
            fee,
        ),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ],
    )?;

    // Переводим основную сумму в джекпот
    invoke(
        &system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.ticket_jackpot.to_account_info().key,
            jackpot_amount,
        ),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.ticket_jackpot.to_account_info(),
        ],
    )?;

    // Добавляем участника в текущий чанк
    current_chunk.participants.push(ctx.accounts.user.key());
    current_chunk.current_count += 1;
    ticket_account.total_participants += 1;

    // Обновляем суммы
    ticket_jackpot.total_amount += jackpot_amount;

    emit!(TicketPurchasedEvent {
        ticket_id,
        user: ctx.accounts.user.key(),
        amount,
        chunk_index: current_chunk.chunk_index,
        participant_index: current_chunk.current_count - 1,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

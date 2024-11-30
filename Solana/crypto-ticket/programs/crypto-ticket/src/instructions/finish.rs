//programs/crypto-ticket/src/instructions/finish.rs
use crate::state::{TicketAccount, TicketJackpot};
use crate::error::ErrorCode;
use crate::events::TicketFinishedEvent;
use crate::log_event;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(ticket_id: u64)]
pub struct FinishTicket<'info> {
    #[account(
        mut,
        seeds = [b"ticket", ticket_id.to_le_bytes().as_ref()],
        bump,
        constraint = ticket_account.is_active @ ErrorCode::TicketNotActive, // Проверка чтобы билет был активным
        constraint = ticket_account.admin == user.key() @ ErrorCode::UnauthorizedAccess // Проверка чтобы пользователь был администратором
    )]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(
        mut,
        seeds = [b"jackpot", ticket_id.to_le_bytes().as_ref()],
        bump,
        constraint = !ticket_jackpot.is_claimed @ ErrorCode::JackpotAlreadyClaimed, // Проверка чтобы джекпот не был уже забран
        constraint = ticket_jackpot.ticket_id == ticket_id @ ErrorCode::InvalidTicketJackpot // Проверка чтобы джекпот соответствовал билету
    )]
    pub ticket_jackpot: Account<'info, TicketJackpot>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn finish_ticket(ctx: Context<FinishTicket>, ticket_id: u64) -> Result<()> {
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &ctx.accounts.ticket_jackpot;

    // Убеждаемся что участники есть
    require!(
        ticket_account.total_participants > 0,
        ErrorCode::NoParticipants
    );

    // Убеждаемся что джекпот не пуст
    require!(
        ticket_jackpot.total_amount > 0,
        ErrorCode::EmptyJackpot
    );

    // Рассчитываем минимальную сумму, которую должен получить администратор
    let expected_min_amount = ticket_account.price
        .checked_mul(ticket_account.total_participants)
        .ok_or(ErrorCode::CalculationError)?
        .checked_mul(90) 
        .ok_or(ErrorCode::CalculationError)?
        .checked_div(100)
        .ok_or(ErrorCode::CalculationError)?;

    // Убеждаемся что джекпот не меньше ожидаемой суммы
    require!(
        ticket_jackpot.total_amount >= expected_min_amount,
        ErrorCode::InvalidJackpotAmount
    );

    // Меняем статус билета
    ticket_account.is_active = false;

    let clock = Clock::get()?;

    // Логируем событие
    log_event!(TicketFinishedEvent {
        ticket_id,
        total_participants: ticket_account.total_participants,
        total_jackpot: ticket_jackpot.total_amount,
        timestamp: clock.unix_timestamp as u64,
    });

    Ok(())
}
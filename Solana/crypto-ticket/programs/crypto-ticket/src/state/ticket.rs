//programs/crypto-ticket/src/state/ticket.rs
use anchor_lang::prelude::*;

// Структуры аккаунтов
#[account]
pub struct TicketAccount {
    pub is_active: bool,
    pub admin: Pubkey,
    pub ticket_id: i64,
    pub price: i64,
    pub total_participants: i64,
}

#[account]
pub struct TicketJackpot {
    pub total_amount: i64,
    pub winner: Pubkey,
    pub ticket_id: i64,
    pub is_claimed: bool,
}

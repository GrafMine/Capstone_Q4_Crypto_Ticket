//programs/crypto-ticket/src/state/ticket.rs
use anchor_lang::prelude::*;

// Структуры аккаунтов
#[account]
pub struct TicketAccount {
    pub is_active: bool,
    pub admin: Pubkey,
    pub ticket_id: u64,
    pub price: u64,
    pub total_participants: u64,
}

#[account]
pub struct TicketJackpot {
    pub total_amount: u64,
    pub winner: Pubkey,
    pub ticket_id: u64,
    pub is_claimed: bool,
}

#[account]
pub struct TicketHistory {
    pub ticket_id: u64,
    pub winner: Pubkey,
    pub amount: u64,
    pub participants_count: u64,
    pub claimed_at: i64,
}

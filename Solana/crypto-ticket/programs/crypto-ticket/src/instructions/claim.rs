//programs/crypto-ticket/src/instructions/claim.rs
use crate::state::{ParticipantsChunk, TicketAccount, TicketJackpot};
use switchboard_on_demand::accounts::RandomnessAccountData;
use crate::events::JackpotClaimEvent;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

pub fn claim_jackpot(ctx: Context<ClaimJackpot>, ticket_id: u64) -> Result<()> {
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;
    let clock = Clock::get()?;

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

    // Получаем данные случайности от Switchboard
    let randomness_data = RandomnessAccountData::parse(
        ctx.accounts.randomness_account_data.data.borrow()
    ).map_err(|_| ErrorCode::InvalidRandomnessData)?;

    let random_value = randomness_data.get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;

    let winner_index = (random_value[0] as u64) % ticket_account.total_participants;
    let chunk_index = winner_index / ParticipantsChunk::CHUNK_SIZE as u64;
    let index_in_chunk = winner_index % ParticipantsChunk::CHUNK_SIZE as u64;

    // Проверяем чанк победителя
    let participants_chunk = &ctx.accounts.winner_participants_chunk;
    require!(
        participants_chunk.chunk_index == chunk_index,
        ErrorCode::InvalidChunkIndex
    );

    // Получаем победителя
    let winner_pubkey = participants_chunk.participants[index_in_chunk as usize].pubkey;
    require!(
        ctx.accounts.winner.key() == winner_pubkey,
        ErrorCode::InvalidWinner
    );

    // Выплата джекпота
    let amount = ticket_jackpot.total_amount;
    **ticket_jackpot.to_account_info().try_borrow_mut_lamports()? -= amount as u64;
    **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += amount as u64;

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
        timestamp: clock.unix_timestamp as u64,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(ticket_id: u64)]
pub struct ClaimJackpot<'info> {
    #[account(
        mut,
        seeds = [b"ticket", ticket_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(
        mut,
        seeds = [b"jackpot", ticket_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_jackpot: Account<'info, TicketJackpot>,

    #[account(
        seeds = [
            b"participants",
            ticket_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub winner_participants_chunk: Account<'info, ParticipantsChunk>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: Winner receives jackpot
    #[account(mut)]
    pub winner: AccountInfo<'info>,

    /// CHECK: Switchboard randomness account
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// функцию для проверки timestamp и получения случайного значения
// pub fn verify_random_value(
//     randomness_data: &RandomnessAccountData,
//     saved_timestamp: u64,
// ) -> Result<[u8; 32]> {
//     // Создаем Clock со старым timestamp
//     let historical_clock = Clock {
//         unix_timestamp: saved_timestamp,  // Наш сохраненный timestamp
//         slot: 0,                         // Эти поля можно заполнить нулями
//         epoch_start_timestamp: 0,        // так как Switchboard использует
//         epoch: 0,                        // только unix_timestamp
//         leader_schedule_epoch: 0,
//     };

//     // Получаем то же самое случайное значение
//     // randomness_data.get_value(&historical_clock)
//     //     .map_err(|_| ErrorCode::InvalidRandomnessData)
// }

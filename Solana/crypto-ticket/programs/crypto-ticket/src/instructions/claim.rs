//programs/crypto-ticket/src/instructions/claim.rs
use crate::state::{ParticipantsChunk, TicketAccount, TicketJackpot};
use crate::events::JackpotClaimEvent;
use crate::error::ErrorCode;
use crate::log_event;
use anchor_lang::prelude::*;

pub fn claim_jackpot<'info>(ctx: Context<'info, 'info, 'info, 'info, ClaimJackpot<'info>>, ticket_id: u64) -> Result<()> {
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;
    
    // Только адин может инициировать выплату джекпота
    require!(
        ticket_account.admin == ctx.accounts.user.key(),
        ErrorCode::UnauthorizedAccess
    );

    // Проверка на активность тикета и статус джекпота
    require!(!ticket_jackpot.is_claimed, ErrorCode::JackpotAlreadyClaimed);
    require!(ticket_account.total_participants > 0, ErrorCode::NoParticipants);
    
    // Расчет расположения победителя. Победитель находится в последнем чанке
    let total_participants = ticket_account.total_participants;
    let total_chunks = (total_participants + ParticipantsChunk::CHUNK_SIZE as u64 - 1) / ParticipantsChunk::CHUNK_SIZE as u64;
    let last_chunk_index = (total_participants - 1) / ParticipantsChunk::CHUNK_SIZE as u64;
    let position_in_last_chunk = (total_participants - 1) % ParticipantsChunk::CHUNK_SIZE as u64;

    // Проверка всех чанков
    let mut participants_count = 0;
    let mut last_timestamp = 0;
    let mut is_winner_verified = false;

    for (i, chunk_account) in ctx.remaining_accounts.iter().enumerate() {
        
        let chunk_data = Account::<ParticipantsChunk>::try_from(chunk_account)?;

        // Убеждаемся, что чанк принадлежит тому же тикету
        require!(
            chunk_data.ticket_id == ticket_id,
            ErrorCode::InvalidChunkData
        );

        // убеждаемся, что чанк находится в последовательности
        require!(
            chunk_data.chunk_index == i as u64,
            ErrorCode::InvalidChunkSequence
        );

        // Проверка последовательности timestamp
        for round in &chunk_data.rounds_history {
            require!(
                round.timestamp >= last_timestamp,
                ErrorCode::InvalidTimestampSequence
            );
            last_timestamp = round.timestamp;
        }

        participants_count += chunk_data.current_count;

        // Проверка победителя
        if chunk_data.chunk_index == last_chunk_index {

            // Недопускаем ситуацию, когда в последнем чанке нет участников
            require!(
                chunk_data.current_count > position_in_last_chunk,
                ErrorCode::InvalidChunkData
            );

            let winner = &chunk_data.participants[position_in_last_chunk as usize];

            // Проверка победителя
            require!(
                ctx.accounts.winner.key() == winner.pubkey,
                ErrorCode::InvalidWinner
            );
            is_winner_verified = true;
        }
    }

    // Проверка наличия всех чанков
    require!(
        ctx.remaining_accounts.len() == total_chunks as usize,
        ErrorCode::MissingChunks
    );

    // Проверка количества участников
    require!(
        participants_count == total_participants,
        ErrorCode::ParticipantCountMismatch
    );

    // Проверка победителя
    require!(is_winner_verified, ErrorCode::WinnerNotVerified);

    // Начинаем выплату джекпота
    let amount = ticket_jackpot.total_amount;
    **ticket_jackpot.to_account_info().try_borrow_mut_lamports()? -= amount as u64;
    **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += amount as u64;

    ticket_jackpot.is_claimed = true;
    ticket_jackpot.winner = ctx.accounts.winner.key();
    ticket_account.is_active = false;

    // Отправляем событие в логчейн
    log_event!(JackpotClaimEvent {
        ticket_id,
        winner: ctx.accounts.winner.key(),
        amount,
        chunk_index: last_chunk_index,
        index_in_chunk: position_in_last_chunk,
        timestamp: Clock::get()?.unix_timestamp as u64,
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
    pub user: Signer<'info>,

    /// CHECK: Winner receives jackpot
    #[account(mut)]
    pub winner: AccountInfo<'info>,

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

use crate::log_event;
//programs/crypto-ticket/src/instructions/ticket.rs
use crate::state::{ParticipantsChunk, Player, Round, TicketAccount, TicketJackpot};
use anchor_lang::solana_program::{program::invoke, system_instruction};
use switchboard_on_demand::accounts::RandomnessAccountData;
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

    /// CHECK: Switchboard randomness account
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// Покупка билета пользователем
pub fn buy_ticket(ctx: Context<BuyTicket>, ticket_id: i64) -> Result<()> {
    // Сначала получаем все AccountInfo, которые нам понадобятся
    let jackpot_info = &ctx.accounts.ticket_jackpot.to_account_info();
    let user_info = &ctx.accounts.user.to_account_info();
    let admin_info = &ctx.accounts.admin.to_account_info();

    // Теперь получаем мутабельные ссылки на аккаунты
    let ticket_account = &mut ctx.accounts.ticket_account;
    let ticket_jackpot = &mut ctx.accounts.ticket_jackpot;
    let current_chunk = &mut ctx.accounts.current_participants_chunk;

    // Проверки
    require!(ticket_account.is_active, ErrorCode::TicketNotActive);
    require!(!ticket_jackpot.is_claimed, ErrorCode::JackpotAlreadyClaimed);
    require!(ticket_account.ticket_id == ticket_jackpot.ticket_id, ErrorCode::InvalidTicketJackpot);

    // Проверяем, не заполнен ли текущий чанк
    if current_chunk.current_count >= ParticipantsChunk::CHUNK_SIZE as i64 {
        return Err(ErrorCode::ChunkIsFull.into());
    }

    // Рассчитываем комиссию (10%)
    let amount = ticket_account.price;
    let fee = amount / 10;
    let jackpot_amount = amount - fee;

    // Переводим комиссию админу используя сохраненные AccountInfo
    invoke(
        &system_instruction::transfer(
            user_info.key,
            admin_info.key,
            fee as u64,
        ),
        &[
            user_info.clone(),
            admin_info.clone(),
        ],
    )?;

    // Переводим основную сумму в джекпот используя сохраненные AccountInfo
    invoke(
        &system_instruction::transfer(
            user_info.key,
            jackpot_info.key,
            jackpot_amount as u64,
        ),
        &[
            user_info.clone(),
            jackpot_info.clone(),
        ],
    )?;

    let clock = Clock::get()?;
    let randomness_data = RandomnessAccountData::parse(
        ctx.accounts.randomness_account_data.data.borrow()
    ).map_err(|_| ErrorCode::InvalidRandomnessData)?;
    let random_value = randomness_data.get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;

    // Генерируем случайное поле для игрока (9 чисел от 0 до 8)
    let mut field = [0i8; 9];
    for i in 0..9 {
        field[i] = (random_value[i % random_value.len()] % 9) as i8;
    }

    // Генерируем случайные значения для раунда
    let index = (random_value[9] % 9) as i8;  // 0-8
    let dir = (random_value[10] % 4) as i8;   // 0-3 (направления)
    let diff = (random_value[11] % 3) as i8;  // 0-2 (сложность)

    // Добавляем участника в текущий чанк
    current_chunk.participants.push(Player {
        pubkey: ctx.accounts.user.key(),
        field
    });
    current_chunk.rounds_history.push(Round {
        timestamp: clock.unix_timestamp,
        index,
        dir,
        diff
    });
    current_chunk.current_count += 1; // тут надо увеличивать на +1
    ticket_account.total_participants += 1;

    // Обновляем суммы
    ticket_jackpot.total_amount += jackpot_amount;

    log_event!(TicketPurchasedEvent {
        ticket_id,
        user_id: ctx.accounts.user.key(),
        user_field: field,
        round_index: index,
        round_dir: dir,
        round_diff: diff,
        amount,
        chunk_index: current_chunk.chunk_index,
        participant_index: current_chunk.current_count - 1,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

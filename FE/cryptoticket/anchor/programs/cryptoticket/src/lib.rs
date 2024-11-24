#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("JC3UXWnrNz41ui4axLPwTqopXTGeZrotKineBtATBjjb");

#[program]
pub mod cryptoticket {
    use super::*;

  pub fn close(_ctx: Context<CloseCryptoticket>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.cryptoticket.count = ctx.accounts.cryptoticket.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.cryptoticket.count = ctx.accounts.cryptoticket.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeCryptoticket>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.cryptoticket.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeCryptoticket<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Cryptoticket::INIT_SPACE,
  payer = payer
  )]
  pub cryptoticket: Account<'info, Cryptoticket>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseCryptoticket<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub cryptoticket: Account<'info, Cryptoticket>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub cryptoticket: Account<'info, Cryptoticket>,
}

#[account]
#[derive(InitSpace)]
pub struct Cryptoticket {
  count: u8,
}

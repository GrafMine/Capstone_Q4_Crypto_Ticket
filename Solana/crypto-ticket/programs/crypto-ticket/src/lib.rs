//programs/crypto-ticket/src/lib.rs
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EMGePWFB5TKj6hDYX1q6tfB6PJNhRLigBVtS7ZNyW5zf");

#[program]
pub mod crypto_ticket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

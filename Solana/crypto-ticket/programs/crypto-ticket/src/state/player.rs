//programs/crypto-ticket/src/state/player.rs
use anchor_lang::prelude::*;

const MAX_PLAYER_FIELD_SIZE: usize = 9;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Player {
    pub pubkey: Pubkey,
    pub field: [u8; 9]
}

impl Player {
    pub const LEN: usize =
        32 + // size pubkey (32 bytes)
        4 +  // vector length field (4 bytes for keep track of the length of the vector)
        1 * MAX_PLAYER_FIELD_SIZE; // size of size_field (9 elements, each per 1 byte)
}

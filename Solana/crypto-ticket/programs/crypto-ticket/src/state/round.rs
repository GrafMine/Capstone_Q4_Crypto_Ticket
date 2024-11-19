//programs/crypto-ticket/src/state/round.rs
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Round {
    pub timestamp: u64, // Clock::get()?.unix_timestamp
    pub index: u8, // 0-8
    pub dir: u8, // 0-3
    pub diff: u8, // 0-1
}

impl Round {
    pub const LEN: usize =
        64 + // size timestamp
        1 + // size index
        1 +  // size dir
        1; // size diff
}


// 0-UP_RIGHT = 'up-right',
// 1-UP_LEFT = 'up-left',
// 2-DOWN_RIGHT = 'down-right',
// 3-DOWN_LEFT = 'down-left',

// return {
// 	indexCell: 1,
// 	dir: DirectionEnum.UP_LEFT,
// 	diff: 1
// };

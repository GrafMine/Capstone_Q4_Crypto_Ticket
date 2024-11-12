use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, Debug)]
pub struct RegistrationEvent {
    pub user: Pubkey,
    pub name: String,
    pub timestamp: i64,
}


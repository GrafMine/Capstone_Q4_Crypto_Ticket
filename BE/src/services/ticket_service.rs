// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     pubkey::Pubkey,
//     signature::{Keypair, Signer},
//     transaction::Transaction,
//     instruction::Instruction,
//     commitment_config::CommitmentConfig,
// };
// use std::str::FromStr;

// pub struct TicketService {
//     rpc_client: RpcClient,
//     admin_keypair: Keypair,
//     program_id: Pubkey,
// }

// impl TicketService {
//     pub fn new(rpc_url: &str, admin_keypair_base58: &str, program_id_str: &str) -> Self {

//         let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
//         let admin_keypair = Keypair::from_base58_string(admin_keypair_base58);
//         let program_id = Pubkey::from_str(program_id_str).expect("Invalid program ID");

//         Self {
//             rpc_client,
//             admin_keypair,
//             program_id,
//         }
//     }

//     pub fn create_lottery(&self, lottery_account: &Keypair) -> Result<String, Box<dyn std::error::Error>> {
        
//         let instruction = Instruction::new_with_bincode::<Vec<u8>>(
//             self.program_id,
//             &vec![],
//             vec![
//                 solana_sdk::instruction::AccountMeta::new(lottery_account.pubkey(), true),
//                 solana_sdk::instruction::AccountMeta::new(self.admin_keypair.pubkey(), true),
//             ],
//         );

//         let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
//         let mut transaction = Transaction::new_with_payer(&[instruction], Some(&self.admin_keypair.pubkey()));
//         transaction.sign(&[&self.admin_keypair, lottery_account], recent_blockhash);

//         let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
//         Ok(signature.to_string())
//     }
// }
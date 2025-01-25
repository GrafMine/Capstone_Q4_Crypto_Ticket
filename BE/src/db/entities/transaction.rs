#[derive(Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_pubkey: String,
    pub ticket_pubkey: String,
    pub amount: f64,
    pub transaction_type: String, // "TransactionType" може бути окремим типом
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct UserTicketHistoryEarnings {
    pub id: i32,
    pub user_pubkey: String,
    pub ticket_pubkey: String,
    pub amount: f64,
    pub total_amount: f64,
    pub from_user_pubkey: String,
    pub status: String, // "EarningStatus" може бути окремим типом
    pub created_at: chrono::NaiveDateTime,
}

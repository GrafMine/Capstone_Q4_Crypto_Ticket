#[derive(Debug)]
pub struct TicketHistory {
    pub id: i32,
    pub ticket_pubkey: String,
    pub user_pubkey: String,
    pub action_time: chrono::NaiveDateTime,
    pub action_type: String, // "TicketHistoryActionType" може бути окремим типом
}

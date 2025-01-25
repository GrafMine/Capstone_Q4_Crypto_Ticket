#[derive(Debug)]
pub struct PlayerTicketScore {
    pub id: i32,
    pub ticket_pubkey: String,
    pub user_pubkey: String,
    pub score: i32,
    pub last_updated: chrono::NaiveDateTime,
}

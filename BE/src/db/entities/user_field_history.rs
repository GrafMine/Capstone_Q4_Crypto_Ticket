#[derive(Debug)]
pub struct UserFieldHistory {
    pub id: i32,
    pub user_pubkey: String,
    pub ticket_pubkey: String,
    pub field_values: serde_json::Value,
    pub move_object: serde_json::Value,
    pub changed_at: chrono::NaiveDateTime,
}

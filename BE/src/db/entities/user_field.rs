#[derive(Debug)]
pub struct UserField {
    pub id: i32,
    pub user_pubkey: String,
    pub ticket_pubkey: String,
    pub inited_field: serde_json::Value,
    pub current_field: serde_json::Value,
    pub updated_at: chrono::NaiveDateTime,
}

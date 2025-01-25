#[derive(Debug)]
pub struct PerfectCombinationsLeaderboard {
    pub id: i32,
    pub user_pubkey: String,
    pub combinations: i32,
    pub rank: i32,
    pub updated_at: chrono::NaiveDateTime,
}

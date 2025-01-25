use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::db::schema::tickets;


#[derive(Queryable, Debug, Insertable, AsChangeset)]
#[diesel(table_name = tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticket {
    pub pubkey: String,
    pub currency: String,
    pub entry_fee: i32,
    pub duration_type: String,
    pub duration: Option<String>,
    pub end_time: Option<NaiveDateTime>,
    pub main_bank_amount: f64,
    pub col_bank_amount: f64,
    pub total_players: i32,
    pub created_at: NaiveDateTime,
    pub current_state: String,
}
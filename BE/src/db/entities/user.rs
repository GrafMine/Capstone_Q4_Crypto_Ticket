use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::db::schema::users;

#[derive(Queryable, Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub pubkey: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub total_earnings: f64,
    pub withdrawn_earnings: f64,
    pub perfect_combinations_won: i32,
    pub created_at: NaiveDateTime,
    pub total_earnings_rank: Option<i32>,
    pub withdrawn_earnings_rank: Option<i32>,
    pub perfect_combinations_rank: Option<i32>,
}
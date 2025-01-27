use diesel::{Connection, PgConnection};

use crate::utils::helpers::get_env_var;

#[cfg(test)]
mod tests {
    use crate::db::{entities::user::User, repositories::user_repository, repositories::repository::Repository};
    use super::*;

    #[test]
    fn create_user() {
        let conn = establish_connection();
        let mut user_repository = user_repository::UserRepository::new(conn);

        let user_id = "123".to_string();

        let create = user_repository.create(User {
            pubkey: user_id.clone(),
            username: Some("Ihor".to_string()),
            email: Some("movigos@gmail.com".to_string()),
            total_earnings: 0.0,
            withdrawn_earnings: 0.0,
            perfect_combinations_won: 0,
            created_at: chrono::Utc::now().naive_utc(),
            total_earnings_rank: Some(0),
            withdrawn_earnings_rank: Some(1),
            perfect_combinations_rank: Some(2),
        });
        assert_eq!(user_id.clone(), create.unwrap().pubkey);

        let remove = user_repository.delete(user_id.clone());
        assert_eq!(Ok(()), remove);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = get_env_var("DATABASE_URL");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

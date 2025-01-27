use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::OptionalExtension;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::db::entities::user::User;
use super::repository::Repository;

pub struct UserRepository {
    conn: PgConnection,
}

impl UserRepository {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }
}

impl Repository<User, String> for UserRepository {
    fn create(&mut self, user: User) -> Result<User, Error> {
        diesel::insert_into(crate::db::schema::User::table)
            .values(user)
            .get_result(&mut self.conn)
    }

    fn find_by_id(&mut self, pubkey: String) -> Result<Option<User>, Error> {
        crate::db::schema::User::table
            .find(pubkey)
            .first(&mut self.conn)
            .optional()
    }

    fn find_all(&mut self) -> Result<Vec<User>, Error> {
        crate::db::schema::User::table.load::<User>(&mut self.conn)
    }

    fn update(&mut self, user: User) -> Result<User, Error> {
        diesel::update(crate::db::schema::User::table.find(&user.pubkey))
            .set(&user)
            .get_result(&mut self.conn)
    }

    fn delete(&mut self, pubkey: String) -> Result<(), Error> {
        diesel::delete(crate::db::schema::User::table.find(pubkey))
            .execute(&mut self.conn)
            .map(|_| ())
    }
}
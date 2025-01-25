use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::OptionalExtension;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::db::entities::user::User;
use crate::db::schema::users;

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
        diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut self.conn)
    }

    fn find_by_id(&mut self, pubkey: String) -> Result<Option<User>, Error> {
        users::table
            .find(pubkey)
            .first(&mut self.conn)
            .optional()
    }

    fn find_all(&mut self) -> Result<Vec<User>, Error> {
        users::table.load::<User>(&mut self.conn)
    }

    fn update(&mut self, user: User) -> Result<User, Error> {
        diesel::update(users::table.find(&user.pubkey))
            .set(&user)
            .get_result(&mut self.conn)
    }

    fn delete(&mut self, pubkey: String) -> Result<(), Error> {
        diesel::delete(users::table.find(pubkey))
            .execute(&mut self.conn)
            .map(|_| ())
    }
}
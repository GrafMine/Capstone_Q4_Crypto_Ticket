use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::OptionalExtension;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::db::entities::ticket::Ticket;

use super::repository::Repository;

pub struct TicketRepository {
    conn: PgConnection,
}

impl TicketRepository {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }
}

impl Repository<Ticket, String> for TicketRepository {
    fn create(&mut self, ticket: Ticket) -> Result<Ticket, Error> {
        diesel::insert_into(crate::db::schema::Ticket::table)
            .values(ticket)
            .get_result(&mut self.conn)
    }

    fn find_by_id(&mut self, pubkey: String) -> Result<Option<Ticket>, Error> {
        crate::db::schema::Ticket::table
            .find(pubkey)
            .first(&mut self.conn)
            .optional()
    }

    fn find_all(&mut self) -> Result<Vec<Ticket>, Error> {
        crate::db::schema::Ticket::table.load::<Ticket>(&mut self.conn)
    }

    fn update(&mut self, ticket: Ticket) -> Result<Ticket, Error> {
        diesel::update(crate::db::schema::Ticket::table.find(&ticket.pubkey))
            .set(&ticket)
            .get_result(&mut self.conn)
    }

    fn delete(&mut self, pubkey: String) -> Result<(), Error> {
        diesel::delete(crate::db::schema::Ticket::table.find(pubkey))
            .execute(&mut self.conn)
            .map(|_| ())
    }
}
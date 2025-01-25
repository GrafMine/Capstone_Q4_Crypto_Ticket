use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::OptionalExtension;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::db::entities::ticket::Ticket;
use crate::db::schema::tickets;

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
        diesel::insert_into(tickets::table)
            .values(ticket)
            .get_result(&mut self.conn)
    }

    fn find_by_id(&mut self, pubkey: String) -> Result<Option<Ticket>, Error> {
        tickets::table
            .find(pubkey)
            .first(&mut self.conn)
            .optional()
    }

    fn find_all(&mut self) -> Result<Vec<Ticket>, Error> {
        tickets::table.load::<Ticket>(&mut self.conn)
    }

    fn update(&mut self, ticket: Ticket) -> Result<Ticket, Error> {
        diesel::update(tickets::table.find(&ticket.pubkey))
            .set(&ticket)
            .get_result(&mut self.conn)
    }

    fn delete(&mut self, pubkey: String) -> Result<(), Error> {
        diesel::delete(tickets::table.find(pubkey))
            .execute(&mut self.conn)
            .map(|_| ())
    }
}
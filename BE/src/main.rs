mod db;
mod events;
mod field;
mod routes;
mod services;
mod utils;
mod tests;

use actix_web::{web, App, HttpServer};
use db::{entities::user::User, repositories::{repository::Repository, user_repository}};
use diesel::{Connection, PgConnection};
use std::{env, sync::Arc};
use ticket_initialized_event::TicketInitializedEvent;
use tokio::sync::{Mutex, RwLock};

use events::*;
use services::pubsub_service::PubSubService;
use utils::helpers::get_env_var;

struct AppState {
    db_conn: Mutex<PgConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut game_field_service = field::game_field::GameFieldService::new(4);

    game_field_service.print_field();
    game_field_service.move_cell(0, field::direction::Direction::TopRight, 1);
    game_field_service.print_field();
    game_field_service.move_cell(3, field::direction::Direction::TopRight, 1);
    game_field_service.print_field();

    game_field_service.move_cell(5, field::direction::Direction::TopRight, 1);
    game_field_service.print_field();

    // Create a new PubSubService instance
    let pubsub_service = Arc::new(RwLock::new(
        PubSubService::new(
            &get_env_var("WS_BLOCKCAIN_HOST"),
            &get_env_var("PROGRAM_ID"),
        )
        .await
        .unwrap(),
    ));
    // Clone pubsub_service for the event listener
    let pubsub_service_clone = pubsub_service.clone();

    // Start the event listener
    tokio::spawn(async move {
        if let Err(e) = on_event_received(pubsub_service_clone).await {
            eprintln!("Error in event listener: {:?}", e);
        }
    });

    let conn = establish_connection();
    let mut user_repository = user_repository::UserRepository::new(conn);
    
    let create = user_repository.create(User {
        pubkey:"123".to_string(),
        username:Some("Ihor".to_string()),
        email:Some("movigos@gmail.com".to_string()), 
        total_earnings: todo!(), 
        withdrawn_earnings: todo!(), 
        perfect_combinations_won: todo!(), 
        created_at: todo!(), 
        total_earnings_rank: todo!(), 
        withdrawn_earnings_rank: todo!(), 
        perfect_combinations_rank: todo!()
    });

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .configure(routes::init)
            .app_data(web::Data::new(pubsub_service.clone()))
    })
    .bind(&get_env_var("SERVER_HOST"))?
    .run()
    .await
}

async fn on_event_received(pubsub_service: Arc<RwLock<PubSubService>>) -> Result<(), Box<dyn std::error::Error>> {
    let pubsub_service = pubsub_service.write().await;

    // Add event listener
    pubsub_service
        .add_event_listener(|event: TicketInitializedEvent| {
            println!("Received registration event: {:?}", event);
        })
        .await?;

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = get_env_var("DATABASE_URL");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

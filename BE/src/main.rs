use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use ticket_initialized_event::TicketInitializedEvent;
use tokio::sync::RwLock;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature, Signer},
};

mod services;
use services::pubsub_service::PubSubService;

mod events;
use events::*;

mod constants;
use constants::{PROGRAM_ID, SERVER_HOST, WS_BLOCKCAIN_HOST};

mod routes;
mod field;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Create a new PubSubService instance
    let pubsub_service = Arc::new(RwLock::new(PubSubService::new(WS_BLOCKCAIN_HOST, PROGRAM_ID).await.unwrap()));
    
    // Clone pubsub_service for the event listener
    let pubsub_service_clone = pubsub_service.clone();

    // Start the event listener
    tokio::spawn(async move {
        if let Err(e) = on_event_received(pubsub_service_clone).await {
            
            eprintln!("Error in event listener: {:?}", e);
        }
    });

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
        .configure(routes::init)
            .app_data(web::Data::new(pubsub_service.clone()))
    })
        .bind(SERVER_HOST)?
        .run()
        .await
}

async fn on_event_received(pubsub_service: Arc<RwLock<PubSubService>>) -> Result<(), Box<dyn std::error::Error>> {
    
    let pubsub_service = pubsub_service.write().await;

    // Add event listener
    pubsub_service.add_event_listener(|event: TicketInitializedEvent| {
        
        println!("Received registration event: {:?}", event);

    }).await?;

    Ok(())
}

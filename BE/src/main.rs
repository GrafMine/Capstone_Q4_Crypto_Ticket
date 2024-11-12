use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use tokio::sync::RwLock;

mod services;
use services::pubsub_service::PubSubService;

mod events;
use events::register_event::RegistrationEvent;

mod constants;
use constants::{PROGRAM_ID, SERVER_HOST, WS_BLOCKCAIN_HOST};

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
            .app_data(web::Data::new(pubsub_service.clone()))
    })
        .bind(SERVER_HOST)?
        .run()
        .await
}

async fn on_event_received(pubsub_service: Arc<RwLock<PubSubService>>) -> Result<(), Box<dyn std::error::Error>> {
    
    let pubsub_service = pubsub_service.write().await;

    // Add event listener
    pubsub_service.add_event_listener("RegistrationEvent", |event: RegistrationEvent| {
        
        println!("Received registration event: {:?}", event);

    }).await?;

    Ok(())
}

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use borsh::BorshDeserialize;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use futures_util::StreamExt;
use std::sync::Arc;

use crate::utils::helpers::get_env_var;


pub struct PubSubService {
    client: Arc<PubsubClient>,
    pub_key: String,
}

impl PubSubService {
    pub async fn new(ws_blockchain: &str, pub_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        
        let client = PubsubClient::new(ws_blockchain).await;

        match client {
            Ok(client) => {
                println!("Connected to the Blockchain PubSub server");
                Ok(Self {
                    client: Arc::new(client),
                    pub_key: pub_key.to_string(),
                })
            },
            Err(e) => {
                Err(Box::new(e))
            },
        }
    }

    pub fn set_program_id(&self, new_pub_key: String) -> Self {
        Self {
            client: Arc::clone(&self.client),
            pub_key: new_pub_key,
        }
    }

    pub async fn add_event_listener<F, T>(&self, action: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(T) + Send + 'static,
        T: std::fmt::Debug + BorshDeserialize + 'static,
    {
        let filter = RpcTransactionLogsFilter::Mentions(vec![self.pub_key.to_string()]);
        
        let config = RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::confirmed()) };

        let (mut logs, _unsubscribe) = self.client.logs_subscribe(filter, config).await?;

        let event_type = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("Unknown");
    
        while let Some(log) = logs.next().await {
            
            let event_prefix = format!("Program log: {}:{}:", get_env_var("LOG_EVENT_KEY"), event_type);
            
            println!("Received log: {:?}", log);
            println!("event_prefix: {:?}", event_prefix);
            if let Some(event) = log.value.logs.into_iter()
                .find(|line| line.contains(&event_prefix))
                .and_then(|line| line.split(&event_prefix).nth(1).map(|s| s.to_string()))
                .map(|data| data.trim().to_string()) {
                    
                    let cleaned_data = event.replace("\"", "");

                    println!("Received event: {}", cleaned_data);
                    let decoded = BASE64_STANDARD.decode(&cleaned_data)?;
                    
                    let registration_event: T = borsh::BorshDeserialize::try_from_slice(&decoded)?;
                    action(registration_event);
                }
        }

        Ok(())
    }
}

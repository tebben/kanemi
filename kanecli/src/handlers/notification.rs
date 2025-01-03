use crate::commands::notifications::NotificationOptions;
use kanemi::dataplatform::models::{
    config::{DatasetConfig, MqttConfig},
    response::NotificationReponse,
};
use kanemi::errors::NotificationError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationMessage {
    pub success: bool,
    pub topic: String,
    pub data: NotificationReponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub success: bool,
    pub error: String,
}

pub async fn handle_command(options: NotificationOptions) {
    let client_id = if let Some(client_id) = options.client_id {
        client_id
    } else {
        Uuid::new_v4().to_string()
    };

    let dataset_config = DatasetConfig::new(options.name, options.version);
    if let Err(e) = run(options.api_key, client_id, dataset_config).await {
        eprintln!("Error: {}", e);
    }
}

pub async fn run(
    api_key: String,
    client_id: String,
    dataset_config: DatasetConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mqtt_config = MqttConfig::new_default(api_key, dataset_config);
    let mut notification_service =
        kanemi::dataplatform::notification::NotificationService::new(mqtt_config);

    let message_handler = Arc::new(|topic: String, payload: NotificationReponse| {
        let message = NotificationMessage {
            success: true,
            topic,
            data: payload,
        };

        print_notification(message);
    });

    let error_handler = Arc::new(|error: NotificationError| {
        let message = ErrorMessage {
            success: false,
            error: error.to_string(),
        };

        print_notification(message);
    });

    notification_service
        .start(client_id.to_string(), false, message_handler, error_handler)
        .await?;

    Ok(())
}

fn print_notification<T: Serialize>(notification: T) {
    let json = serde_json::to_string(&notification).unwrap();
    println!("{}", json);
}

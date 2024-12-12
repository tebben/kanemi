use crate::commands::notifications::NotificationOptions;
use kanemi::dataplatform::{
    models::config::DatasetConfig, models::config::MqttConfig,
    models::response::NotificationReponse,
};
use std::sync::Arc;

pub async fn handle_command(options: NotificationOptions) {
    let dataset_config = DatasetConfig::new("radar_forecast".to_string(), "2.0".to_string());
    if let Err(e) = run_notification_test(options.api_key, dataset_config).await {
        eprintln!("Error: {}", e);
    }
}

pub async fn run_notification_test(
    api_key: String,
    dataset_config: DatasetConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Waiting for notifications...");

    let mqtt_config = MqttConfig::new_default(api_key, dataset_config);
    let mut notification_service =
        kanemi::dataplatform::notification::NotificationService::new(mqtt_config);

    let message_handler = Arc::new(|topic: String, payload: NotificationReponse| {
        println!(
            "Received message on topic '{}': {:?}",
            topic, payload.data.url
        );
    });

    notification_service.start(message_handler).await?;

    Ok(())
}

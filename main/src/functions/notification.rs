use knmi_nowcast::dataplatform::{
    models::config::DatasetConfig, models::config::MqttConfig,
    models::response::NotificationReponse,
};
use std::sync::Arc;

pub async fn run_notification_test(
    api_key: String,
    dataset_config: DatasetConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Waiting for notifications...");

    let mqtt_config = MqttConfig::new_default(api_key, dataset_config);
    let mut notification_service =
        knmi_nowcast::dataplatform::notification::NotificationService::new(mqtt_config);

    let message_handler = Arc::new(|topic: String, payload: NotificationReponse| {
        println!(
            "Received message on topic '{}': {:?}",
            topic, payload.data.url
        );
    });

    notification_service.start(message_handler).await?;

    Ok(())
}

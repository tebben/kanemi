use super::models::config::MqttConfig;
use super::models::response::NotificationReponse;
use rumqttc::{AsyncClient, MqttOptions, QoS, TlsConfiguration, Transport};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

type NotificationMessageHandler = Arc<dyn Fn(String, NotificationReponse) + Send + Sync>;

pub struct NotificationService {
    mqtt_config: MqttConfig,
    stop_flag: Arc<AtomicBool>,
}

impl NotificationService {
    pub fn new(mqtt_config: MqttConfig) -> Self {
        NotificationService {
            mqtt_config,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    pub async fn start(
        &mut self,
        handler: NotificationMessageHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut mqttoptions = MqttOptions::new(
            Uuid::new_v4(),
            &self.mqtt_config.broker,
            self.mqtt_config.port,
        );
        mqttoptions.set_transport(Transport::Wss(TlsConfiguration::default()));
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        mqttoptions.set_credentials("kanemi", &self.mqtt_config.api_key);
        mqttoptions.set_clean_session(true);

        // Declare eventloop as mutable
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Construct the topic string dynamically
        let topic = format!(
            "{}/{}/{}/#",
            self.mqtt_config.topic_base,
            self.mqtt_config.dataset_config.dataset_name,
            self.mqtt_config.dataset_config.version
        );

        client.subscribe(&topic, QoS::AtMostOnce).await.unwrap();

        // Continuously listen for incoming messages and pass them to the handler
        while !self.stop_flag.load(Ordering::Relaxed) {
            match eventloop.poll().await {
                Ok(notification) => {
                    if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) =
                        notification
                    {
                        let payload_json = String::from_utf8_lossy(&publish.payload);
                        if let Ok(notification) =
                            serde_json::from_str::<NotificationReponse>(&payload_json)
                        {
                            handler(publish.topic, notification);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error polling eventloop: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}

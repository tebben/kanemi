#[derive(Debug)]
pub struct DatasetConfig {
    pub dataset_name: String,
    pub version: String,
}

impl Clone for DatasetConfig {
    fn clone(&self) -> Self {
        DatasetConfig {
            dataset_name: self.dataset_name.clone(),
            version: self.version.clone(),
        }
    }
}

impl DatasetConfig {
    pub fn new(dataset_name: String, version: String) -> Self {
        DatasetConfig {
            dataset_name,
            version,
        }
    }
}

#[derive(Debug)]
pub struct MqttConfig {
    pub api_key: String,
    pub broker: String,
    pub port: u16,
    pub topic_base: String,
    pub dataset_config: DatasetConfig,
}

impl MqttConfig {
    pub fn new_default(api_key: String, dataset_config: DatasetConfig) -> Self {
        MqttConfig {
            api_key,
            dataset_config,
            broker: "wss://mqtt.dataplatform.knmi.nl".to_string(),
            port: 443,
            topic_base: "dataplatform/file/v1".to_string(),
        }
    }

    pub fn new(
        api_key: String,
        dataset_config: DatasetConfig,
        broker: String,
        port: u16,
        topic_base: String,
    ) -> Self {
        MqttConfig {
            api_key,
            dataset_config,
            broker,
            port,
            topic_base,
        }
    }
}

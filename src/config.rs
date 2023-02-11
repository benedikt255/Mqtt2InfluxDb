use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigData {
    #[allow(non_snake_case)]
    pub InfluxDb: InfluxDbConf,
    #[allow(non_snake_case)]
    pub Mqtt: MqttConf,
    #[allow(non_snake_case)]
    #[serde(default = "default_timeout")]
    pub waitTime: u64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxDbConf {
    #[allow(non_snake_case)]
    pub ServerUri: String,
    #[allow(non_snake_case)]
    pub Token: String,
    #[allow(non_snake_case)]
    pub Bucket: String,
    #[allow(non_snake_case)]
    pub Org:  String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MqttConf {
    #[allow(non_snake_case)]
    pub ServerUri: String,
    #[allow(non_snake_case)]
    #[serde(default = "default_mqtt_port")]
    pub ServerPort: u16,
    #[allow(non_snake_case)]
    pub Topics:  Vec<String>,
    #[allow(non_snake_case)]
    #[serde(default = "default_timeout")]
    pub brokerConnTimeout: u64,
}

fn default_timeout() -> u64 {
    10
}

fn default_mqtt_port() -> u16 {
   1883
}

use serde::{Deserialize, Serialize};


 #[derive(Serialize, Deserialize, Debug)]
 pub struct ConfigData {
    pub InfluxDb: InfluxDbConf,
    pub Mqtt: MqttConf,
    #[serde(default = "default_timeout")]
    pub waitTime: u64,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct InfluxDbConf {
    pub ServerUri: String,
    pub Token: String,
    pub Bucket: String,
    pub Org:  String,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct MqttConf {
    pub ServerUri: String,
    #[serde(default = "default_mqtt_port")]
    pub ServerPort: u16,
    pub Topics:  Vec<String>,
    #[serde(default = "default_timeout")]
    pub brokerConnTimeout: u64,
 }

 fn default_timeout() -> u64 {
    10
}

fn default_mqtt_port() -> u16 {
   1883
}
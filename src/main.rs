pub mod config;

use env_logger::Env;
use influxdb2::Client;
use rumqttc::{AsyncClient, ConnAck, ConnectReturnCode, MqttOptions, Packet, Publish, QoS};
use std::fs::File;
use std::io::BufReader;
use std::str;
use std::time::{Duration};
use std::thread;
use tokio;

use crate::config::{ConfigData};

const DFLT_CLIENT:&str = "Mqtt2InfluxDb";
const QOS:QoS = QoS::AtMostOnce;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    //get conf
    log::info!("Mqtt2InfluxDb started\n\t");
    let file = File::open("config/appsettings.json").expect("file not found");
    let reader = BufReader::new(file);
    let config: ConfigData = serde_json::from_reader(reader).expect("error while reading or parsing");

    log::info!("config read, wait configured time\n\t");
  
    thread::sleep(Duration::from_secs(config.waitTime));

    log::info!("wait finished, connect\n\t");

    //connect to influxdb
    log::debug!("Connecting to InfluxDB server: {}", config.InfluxDb.ServerUri);

    let influxdb = Client::new(config.InfluxDb.ServerUri, &config.InfluxDb.Org, config.InfluxDb.Token);

    //conenct to mqtt broker
    let mut mqttoptions = MqttOptions::new(DFLT_CLIENT.to_string(), config.Mqtt.ServerUri, config.Mqtt.ServerPort);
        mqttoptions.set_keep_alive(Duration::from_secs(config.Mqtt.brokerConnTimeout));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    for topic in config.Mqtt.Topics {
        client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    }

    println!("connected, enter loop\n\t");

    //infinite loop
    loop {
        let notification = eventloop.poll().await.unwrap();
        log::info!("Received = {:?}", notification);
        
        match notification {
            rumqttc::Event::Incoming(Packet::ConnAck(ConnAck {
                code: ConnectReturnCode::Success,
                ..
            })) => {
                log::info!("Connected to MQTT");
            }
            rumqttc::Event::Incoming(Packet::Publish(Publish { topic, payload, .. })) => {
                let line = topic + " value=" + str::from_utf8(&payload).expect("invalid mqtt message");
                if let Err(e) = influxdb.write_line_protocol(&config.InfluxDb.Org, &config.InfluxDb.Bucket, line).await
                    {
                        log::error!("Failed to query InfluxDB: {:?}", e);
                    }
            }
            rumqttc::Event::Incoming(_) => {}
            rumqttc::Event::Outgoing(_) => {}
        }
    }
}
use log::{error, info};
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "name_consumer_group")
        .set("bootstrap.servers", "kafka:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()?;

    consumer.subscribe(&["names_topic"])?;

    info!("Started consuming");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(name) = message.payload_view::<str>() {
                    match name {
                        Ok(name) => println!("Hello {}", name),
                        Err(e) => error!("Error parsing message payload: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Error while receiving message: {:?}", e),
        }
    }
}

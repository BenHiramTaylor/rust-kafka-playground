use poem::{
    handler, post, EndpointExt, Route, Server,
    web::{Json, Data},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use log::info;
use uuid::Uuid;
use chrono::Utc;

#[derive(Deserialize, Serialize)]
struct NameMessage {
    name: String,
}

#[handler]
async fn handle_name(
    Json(name_message): Json<NameMessage>,
    db_pool: Data<&PgPool>,
    producer: Data<&FutureProducer>,
) -> Result<String, anyhow::Error> {
    // Generate UUID and get current timestamp
    let id = Uuid::new_v4();
    let processed_on = Utc::now();

    // Log to database
    sqlx::query!(
        "INSERT INTO Name (id, name, processed_on) VALUES ($1, $2, $3)",
        id,
        name_message.name,
        processed_on
    )
        .execute(db_pool.0)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    // Send to Kafka
    producer
        .send(
            FutureRecord::to("names_topic")
                .payload(&name_message.name)
                .key(&name_message.name),
            Duration::from_secs(0),
        )
        .await
        .map_err(|(e, _)| anyhow::Error::from(e))?;

    info!("Processed name: {}", name_message.name);
    Ok(format!("Name '{}' processed", name_message.name))
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error>  {
    env_logger::init();
    dotenv::dotenv().ok();

    let db_url= std::env::var("DATABASE_URL")?;

    // Database connection
    let db_pool = PgPool::connect(&db_url).await?;

    // Kafka producer
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", "kafka:9092")
        .create()?;

    let app = Route::new().at("/name", post(handle_name))
        .data(db_pool)
        .data(producer);

    Server::new(poem::listener::TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;
    Ok(())
}

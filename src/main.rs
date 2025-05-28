use dotenvy::dotenv;
use iepuras::*;
use std::env;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() -> Result<()> {
    //load env, setup tracing
    dotenv().ok();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let mut args = env::args();
    let _ = args.next(); //first arg is the program name
    if let Some(ops) = args.next() {
        match &ops[..] {
            "upload" => upload().await?,
            "download" => download().await?,
            _ => panic!("Use with <upload | download> param"),
        }
    } else {
        panic!("Use with <upload | download> param");
    }
    Ok(())
}

async fn upload() -> Result<()> {
    //load env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let rabbit_host = env::var("RABBIT_HOST").expect("RABBIT_HOST must be set");
    let rabbit_port = env::var("RABBIT_PORT").expect("RABBIT_PORT must be set");
    let rabbit_user = env::var("RABBIT_USER").expect("RABBIT_USER must be set");
    let rabbit_pass = env::var("RABBIT_PASS").expect("RABBIT_PASS must be set");
    let rabbit_stream = env::var("RABBIT_STREAM").expect("RABBIT_STREAM must be set");
    let rabbit_vhost = env::var("RABBIT_VHOST").expect("RABBIT_VHOST must be set");

    let producer = uploader::MysqlProducer::connect(&database_url)?;
    let consumer = uploader::RabbitConsumer::create(
        &rabbit_host,
        rabbit_port.parse().unwrap(),
        &rabbit_user,
        &rabbit_pass,
        &rabbit_vhost,
        &rabbit_stream,
    )
    .await?;

    let w = Worker::new("MySql_RabbitMQ", producer, consumer);
    w.run().await;

    Ok(())
}

async fn download() -> Result<()> {
    //load env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let rabbit_host = env::var("RABBIT_HOST").expect("RABBIT_HOST must be set");
    let rabbit_port = env::var("RABBIT_PORT").expect("RABBIT_PORT must be set");
    let rabbit_user = env::var("RABBIT_USER").expect("RABBIT_USER must be set");
    let rabbit_pass = env::var("RABBIT_PASS").expect("RABBIT_PASS must be set");
    let rabbit_stream = env::var("RABBIT_STREAM").expect("RABBIT_STREAM must be set");
    let rabbit_vhost = env::var("RABBIT_VHOST").expect("RABBIT_VHOST must be set");

    let consumer = downloader::DummyConsumer {};
    let producer = downloader::RabbitProducer::create(
        &rabbit_host,
        rabbit_port.parse().unwrap(),
        &rabbit_user,
        &rabbit_pass,
        &rabbit_vhost,
        &rabbit_stream,
    )
    .await?;

    let w = Worker::new("RabbitMQ_MySQL", producer, consumer);
    w.run().await;

    Ok(())
}

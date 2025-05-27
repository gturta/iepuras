use dotenvy::dotenv;
use iepuras::*;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() -> Result<()> {
    //load env, setup tracing
    dotenv().ok();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let p = MysqlProducer::connect()?;
    let c = DummyConsumer {};

    let w = Worker::new("SQL", p, c);
    w.run().await;

    Ok(())
}

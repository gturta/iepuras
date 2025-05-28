use super::*;
use crate::*;
use rabbitmq_stream_client::types::Message as RabbitMessage;
use rabbitmq_stream_client::{Environment, NoDedup};
use serde_json;

pub struct RabbitConsumer {
    writer: rabbitmq_stream_client::Producer<NoDedup>,
}

impl RabbitConsumer {
    pub async fn create(
        host: &str,
        port: u16,
        user: &str,
        pass: &str,
        vhost: &str,
        stream: &str,
    ) -> Result<Self> {
        let environment = Environment::builder()
            .host(host)
            .port(port)
            .username(user)
            .password(pass)
            .virtual_host(vhost)
            .build()
            .await?;
        let writer = match environment.producer().build(stream).await {
            Ok(writer) => writer,
            Err(err) => {
                return Err(Error::Local(
                    format!("RabbitMQ writer build error: {}", err).into(),
                ));
            }
        };
        Ok(RabbitConsumer { writer })
    }
}

impl Consumer for RabbitConsumer {
    type Item = Message;

    async fn consume(&self, value: Self::Item) {
        //consume actually writes the value to rabbitmq
        if let Ok(serialized) = serde_json::to_string(&value) {
            let msg = RabbitMessage::builder().body(serialized).build();
            match self.writer.send_with_confirm(msg).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("RabbitMQ write error: {}", err);
                }
            }
        }
    }
}

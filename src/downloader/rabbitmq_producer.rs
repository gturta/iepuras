use super::*;
use crate::*;
use futures::StreamExt;
use rabbitmq_stream_client::{Environment, types::OffsetSpecification};
use serde_json;

pub struct RabbitProducer {
    reader: rabbitmq_stream_client::Consumer,
}

impl RabbitProducer {
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
        let reader = match environment
            .consumer()
            .offset(OffsetSpecification::First)
            .build(stream)
            .await
        {
            Ok(reader) => reader,
            Err(err) => {
                return Err(Error::Local(
                    format!("RabbitMQ reader build error: {}", err).into(),
                ));
            }
        };
        Ok(RabbitProducer { reader })
    }
}

impl Producer for RabbitProducer {
    type Item = Message;

    async fn produce(&mut self) -> Option<Self::Item> {
        //consume actually writes the value to rabbitmq
        if let Some(Ok(delivery)) = self.reader.next().await {
            if let Some(data) = delivery.message().data() {
                let value: Message =
                    serde_json::from_slice(data).expect("Could not unpack message");
                return Some(value);
            }
        }
        None
    }
}

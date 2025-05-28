use super::*;
use crate::*;

pub struct DummyConsumer {}
impl Consumer for DummyConsumer {
    type Item = Message;

    async fn consume(&self, value: Self::Item) {
        tracing::info!("consumed: {:?}", value);
    }
}

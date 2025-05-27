use tokio::sync::mpsc;

pub trait Producer {
    type Item;
    fn produce(&mut self) -> Option<Self::Item>;
}
pub trait Consumer {
    type Item;
    fn consume(&self, value: Self::Item);
}

pub struct Worker<T, P, C> {
    id: String,
    tx_channel: mpsc::Sender<T>,
    rx_channel: mpsc::Receiver<T>,
    producer: P,
    consumer: C,
}

impl<T, P, C> Worker<T, P, C>
where
    T: Send + 'static,
    P: Producer<Item = T> + Send + 'static,
    C: Consumer<Item = T> + Send + 'static,
{
    pub fn new(id: &str, producer: P, consumer: C) -> Self {
        //setup comm channel
        let (tx, rx) = mpsc::channel(50);
        Worker {
            id: id.to_owned(),
            tx_channel: tx,
            rx_channel: rx,
            producer,
            consumer,
        }
    }

    pub async fn run(self) {
        //start producer task
        let id = self.id.clone();
        let producer = tokio::spawn(async move {
            Self::producer_thread(id, self.producer, self.tx_channel).await;
        });
        //start consumer task
        let consumer = tokio::spawn(async move {
            Self::consumer_thread(self.id, self.consumer, self.rx_channel).await;
        });

        let _ = producer.await;
        let _ = consumer.await;
    }

    async fn consumer_thread(id: String, consumer: C, mut rx: mpsc::Receiver<T>) {
        tracing::info!("Worker {} consumer thread starting", id);
        let mut count: usize = 0;
        //read from communication queue
        while let Some(msg) = rx.recv().await {
            count += 1;
            consumer.consume(msg);
        }
        tracing::info!(
            "Worker {} consumer thread ending after {} messages",
            id,
            count
        );
    }

    async fn producer_thread(id: String, mut producer: P, tx: mpsc::Sender<T>) {
        tracing::info!("Worker {} producer thread starting", id);
        let mut count: usize = 0;
        while let Some(msg) = producer.produce() {
            tx.send(msg).await.expect(&format!(
                "Worker {} panic: cannot write to communication queue",
                id
            ));
            count += 1;
        }
        tracing::info!(
            "Worker {} producer thread ending after {} messages",
            id,
            count
        );
    }
}

use crate::*;
use diesel::prelude::*;
use std::env;

#[derive(Debug)]
pub struct Message {
    name: String,
    email: String,
}

pub struct MysqlProducer {
    connection: MysqlConnection,
    results: Option<Vec<Mesaj>>,
    result_index: usize,
    query_limit: i64,
    query_offset: i64,
}
impl Producer for MysqlProducer {
    type Item = Message;

    fn produce(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.next() {
            return Some(value);
        }
        //try to fetch a new page if you have no data to return
        let fetched = self.fetch_next_page().expect("Error loading mesaje");
        if fetched > 0 {
            if let Some(value) = self.next() {
                return Some(value);
            }
        }
        None
    }
}

impl MysqlProducer {
    pub fn connect() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = MysqlConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Ok(Self {
            connection,
            results: None,
            result_index: 0,
            query_limit: 20,
            query_offset: 0,
        })
    }
    fn next(&mut self) -> Option<Message> {
        if let Some(results) = &self.results {
            if let Some(value) = results.get(self.result_index) {
                self.result_index += 1;
                return Some(Message {
                    name: value.name.clone().unwrap_or("".into()),
                    email: value.email.clone().unwrap_or("".into()),
                });
            }
        }
        None
    }

    fn fetch_next_page(&mut self) -> Result<usize> {
        match mesaje::table
            .select(Mesaj::as_select())
            .limit(self.query_limit)
            .offset(self.query_offset)
            .load(&mut self.connection)
        {
            Ok(results) => {
                //store results, update offset
                let returned_rows = results.len();
                self.result_index = 0;
                self.query_offset += returned_rows as i64;
                self.results = Some(results);
                Ok(returned_rows)
            }
            Err(err) => {
                tracing::error!("Error fetchin from table: {}", err);
                return Err(Error::Diesel(err));
            }
        }
    }
}

pub struct DummyConsumer {}
impl Consumer for DummyConsumer {
    type Item = Message;

    fn consume(&self, value: Self::Item) {
        tracing::info!("Dummy consumer consumed: {:?}", value);
    }
}

use rabbitmq_stream_client::error::ClientError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Local(String),
    RabbitMQ(ClientError),
    Diesel(diesel::result::Error),
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Local(err) => write!(f, "{}", err),
            Error::RabbitMQ(err) => err.fmt(f),
            Error::Diesel(err) => err.fmt(f),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Local(value.into())
    }
}

impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        Error::RabbitMQ(value.into())
    }
}
impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::Diesel(value.into())
    }
}

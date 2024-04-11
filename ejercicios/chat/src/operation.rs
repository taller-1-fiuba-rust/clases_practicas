use std::io::Write;

use crate::message;

pub enum Operation {
    Connect(Connection),
    Message {
        from: String,
        message: message::Message,
    },
    Disconnect(String),
}

pub struct Connection {
    pub name: String,
    pub stream: Box<dyn Write + Send + Sync>,
}

use std::{
    io::{stdin, Read},
    net::TcpStream,
    thread,
};

use crate::{error::ChatError, message::Message};

pub fn run_client(server_address: &str) -> Result<(), ChatError> {
    // Intentamos conectar el socket a un puerto abierto
    let mut socket = TcpStream::connect(server_address).map_err(|e| ChatError::NetworkError(e))?;
    let mut read_socket = socket.try_clone()?;

    thread::spawn(move || process_messages(&mut read_socket));

    for line in stdin().lines() {
        let message = match line {
            Ok(l) => Message { content: l },
            Err(_) => return Err(ChatError::MessageError),
        };

        message
            .write_to(&mut socket)
            .map_err(|e| ChatError::NetworkError(e))?;
    }

    Ok(())
}

fn process_messages(stream: &mut dyn Read) {
    while let Ok(message) = Message::read_from(stream) {
        println!("{}", message.content);
    }
}

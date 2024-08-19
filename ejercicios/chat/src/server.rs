use crate::{
    error::ChatError,
    message::Message,
    operation::{Connection, Operation},
};
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub fn run_server(server_address: &str) -> Result<(), ChatError> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || process_messages(rx));

    let listener = TcpListener::bind(server_address)?;

    for client_stream in listener.incoming() {
        let stream = client_stream?;
        let thread_tx = tx.clone();
        thread::spawn(move || handle_client(stream, thread_tx));
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, sender: Sender<Operation>) -> Result<(), ChatError> {
    let name = Message::read_from(&mut stream)?.content;
    println!("Usuario recibido: {:?}", name);
    let write_stream = Box::new(stream.try_clone()?);
    sender
        .send(Operation::Connect(Connection {
            name: name.clone(),
            stream: write_stream,
        }))
        .map_err(|_| ChatError::InternalError)?;

    while let Ok(message) = Message::read_from(&mut stream) {
        println!("Mensaje recibido: {:?} de {}", message, name);
        sender
            .send(Operation::Message {
                message,
                from: name.clone(),
            })
            .map_err(|_| ChatError::InternalError)?;
    }

    sender
        .send(Operation::Disconnect(name.clone()))
        .map_err(|_| ChatError::InternalError)?;
    Ok(())
}

fn process_messages(rx: Receiver<Operation>) -> Result<(), ChatError> {
    let mut conns: Vec<Connection> = Vec::new();

    for received in rx {
        match received {
            Operation::Connect(c) => conns.push(c),
            Operation::Message { from, mut message } => {
                message.content = format!("{}: {}", from, message.content);
                for client in conns.iter_mut() {
                    println!("Reenviando: {:?}", message);
                    message.write_to(&mut client.stream)?;
                }
            }
            Operation::Disconnect(name) => {
                conns = conns.into_iter().filter(|c| c.name != name).collect();
            }
        }
    }

    Ok(())
}

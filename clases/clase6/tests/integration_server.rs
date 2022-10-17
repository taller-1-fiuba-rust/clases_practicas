use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread::{self, JoinHandle},
    time::Duration,
};

use clase_testing::server::Server;

#[test]
fn int_handshake_ok() -> std::io::Result<()> {
    let addrs = "127.0.0.1:8082";

    let server = Server {
        address: addrs.to_string(),
    };
    let handle = thread::spawn(move || {
        if let Err(_) = server.run() {
            panic!("The server execution failed")
        }
    });

    thread::sleep(Duration::from_millis(500));

    let mut socket = TcpStream::connect(addrs)?;

    let handshake = "Handshake initiated!\n";

    // inicio el handshake
    socket.write(handshake.as_bytes())?;

    println!("handshake enviado!");

    // leo la respuesta del handshake
    let mut response = String::new();
    let mut reader = BufReader::new(socket);
    let line_bytes = reader.read_line(&mut response)?;

    println!(
        "mensaje recibido - bytes: {}, content: {}",
        line_bytes, response
    );

    assert_eq!(line_bytes, 21);
    assert_eq!(response, String::from("Handshake completed!\n"));

    match handle.join() {
        Ok(_) => Ok(()),
        Err(_) => panic!("Se esperaba que el thread termine correctamente"),
    }
}

#[test]
fn int_handshake_invalid() -> std::io::Result<()> {
    let addrs = "127.0.0.1:8081";

    let server = Server {
        address: addrs.to_string(),
    };
    let handle: JoinHandle<std::io::Result<()>> = thread::spawn(move || server.run());

    thread::sleep(Duration::from_millis(500));

    let mut socket = TcpStream::connect(addrs)?;

    let handshake = "Handshake invalido...\n";

    // inicio el handshake
    socket.write(handshake.as_bytes())?;

    println!("handshake invalido enviado!");

    match handle.join() {
        Ok(res) => match res {
            Ok(_) => panic!("Se esperaba que el server devuelva un Err!"),
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
        },
        Err(_) => panic!("Se esperaba que el thread termine correctamente"),
    }

    Ok(())
}

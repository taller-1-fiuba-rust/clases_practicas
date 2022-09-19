//! Abre un puerto TCP en el puerto asignado por argv.
//! Escribe las lineas recibidas a stdout y las manda mediante el socket.

use std::env::args;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};

static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("Usage:\n{:?} <puerto>", app_name);
        return Err(());
    }

    let address = "127.0.0.1:".to_owned() + &argv[1];
    server_run(&address).unwrap();
    Ok(())
}

fn server_run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
    let (mut client_stream, socket_addr) = listener.accept()?;
    println!("La socket addr del client: {:?}", socket_addr);
    // let mut client_stream : TcpStream = connection.0;
    // TcpStream implementa el trait Read, así que podemos trabajar como si fuera un archivo
    handle_client(&mut client_stream)?;
    Ok(())
}

// 
fn handle_client(stream: &mut dyn Read) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    // iteramos las lineas que recibimos de nuestro cliente
    while let Some(Ok(line)) = lines.next() {
        println!("Recibido: {:?}", line);
    }
    Ok(())
}

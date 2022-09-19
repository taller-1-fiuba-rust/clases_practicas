//! Abre un puerto TCP en el puerto asignado por argv.
//! Escribe las lineas recibidas a stdout y las manda mediante el socket.
mod alumno;
use std::io::Read;
use alumno::Alumno;
use std::env::args;
use std::net::{TcpListener, TcpStream};


static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }
    
    let address = "0.0.0.0:".to_owned() + &argv[1];
    server_run(&address).unwrap();
    Ok(())
}

// fn server_run(address: &str) -> std::io::Result<()> {
//     let listener = TcpListener::bind(address)?;
//     // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
//     let connection = listener.accept()?;
//     let mut client_stream : TcpStream = connection.0;
//     // TcpStream implementa el trait Read, así que podemos trabajar como si fuera un archivo
//     handle_client(&mut client_stream)?;
//     Ok(())
// }

fn handle_client(stream: &mut dyn Read) -> std::io::Result<()> {
    let alumno = Alumno::read_from(stream);
    println!("Recibió {:?}", alumno);
    Ok(())
}

fn server_run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
    for client_stream in listener.incoming() {
        handle_client(&mut client_stream.unwrap())?;
    }
    Ok(())
}




//! Se conecta mediante TCP a la dirección asignada por argv.
//! Lee lineas desde stdin y las manda mediante el socket.
mod alumno;
use alumno::Alumno;
use std::env::args;
use std::net::TcpStream;

static CLIENT_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }

    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);

    client_run(&address).unwrap();
    Ok(())
}

fn client_run(address: &str) -> std::io::Result<()> {
    // Vamos a mandar datos crudos
    let mut socket = TcpStream::connect(address)?;
    let alumno = Alumno {nombre: "Pepe Muleiro".to_owned(), padron: 80880};
    println!("Enviando: {:?}", alumno);
    alumno.write_to(&mut socket).unwrap();
    Ok(())
}

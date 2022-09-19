//! Se conecta mediante TCP a la dirección asignada por argv.
//! Lee lineas desde stdin y las manda mediante el socket.

use std::env::args;
use std::io::stdin;
use std::io::Write;
use std::io::{BufRead, BufReader, Read};
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


    client_run(&address, &mut stdin()).unwrap();
    Ok(())
}

/// Client run recibe una dirección y cualquier cosa "legible"
/// Esto nos da la libertad de pasarle stdin, un archivo, incluso otro socket
fn client_run(address: &str, stream: &mut dyn Read) -> std::io::Result<()> {
    // Vamos a usar un BufReader para comodidad de leer lineas
    // Notar que como el stream es de tipo `Read`, podemos leer de a bytes.
    // BufReader nos provee una capa de abstracción extra para manejarnos con strings
    let reader = BufReader::new(stream);
    // Intentamos conectar el socket a un puerto abierto
    let mut socket = TcpStream::connect(address)?;
    // BufReader nos permite leer lineas de texto
    for line in reader.lines() {
        // lines nos devuelve un iterador de Result(string), agarramos el string adentro
        if let Ok(line) = line {
            println!("Enviando: {:?}", line);
            // TcpStream implementa Write
            socket.write(line.as_bytes())?;
            // El reader le quita el salto de linea, así que se lo mando aparte
            socket.write("\n".as_bytes())?;
        }
    }
    Ok(())
}

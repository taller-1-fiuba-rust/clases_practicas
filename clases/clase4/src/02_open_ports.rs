//! Reporta al usuario la lista de puertos abiertos del host.
//! Toma la cantidad de threads y la IP del host al que se intentara conectar.
//! Para ejecutar: cargo run --bin 02-open_ports 2 127.0.0.1

use core::num;
use std::{env, process};
use std::net::{IpAddr, TcpStream}; 
use std::thread;
use std::sync::mpsc::{Sender, channel};

const CLIENT_ARGS: usize = 3;
const MAX_PORT: u16 = 65535;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &args[0];
        println!("{:?} <cantidad de hilos> <host>", app_name);
        process::exit(1);
	}

    // FromStr
	let num_threads: u16 = args[1].clone().parse().unwrap();
	let ipaddr: IpAddr = args[2].clone().parse().unwrap();
    
	let (tx, rx) = channel();

	for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan_ports(tx, i, ipaddr, num_threads);
        });
	}

	drop(tx); // libero tx, para terminar la espera de mensajes.

	let mut ports = vec![];
    for received_port in rx {
        ports.push(received_port);
    }    

    print!("\n");

    for p in ports {
        println!("{} is open!", p);
    }
}

fn scan_ports(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
	let mut current_port: u16 = start_port + 1;

	loop {
        println!("Escaneando el puerto {:?}", current_port);
		match TcpStream::connect((addr, current_port)) {
			Ok(_) => { // me pude conectar, el puerto esta abierto
				print!(".");
            
				tx.send(current_port).unwrap();
			},
			Err(_) => {}
		}

		if (MAX_PORT - current_port) <= num_threads {
			break;
		}
		current_port += num_threads;
	}
}
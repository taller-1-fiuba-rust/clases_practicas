use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let timeout = Duration::from_secs(3);

    let (tx, rx) = mpsc::channel();

    let shared_tx1 = Arc::new(Mutex::new(tx));
    let shared_tx2 = shared_tx1.clone();

    // lectura del archivo de palabras en ingles
    let en_producer_handle = thread::spawn(move || {
        let file = File::open("data/data_en").unwrap();
        for line in BufReader::new(file).lines() {
            if let Ok(s) = line {
                shared_tx1.lock().unwrap().send(s).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    // lectura del archivo de palabras en castellano
    let sp_producer_handle = thread::spawn(move || {
        let file = File::open("data/data_sp").unwrap();
        for line in BufReader::new(file).lines() {
            if let Ok(s) = line {
                shared_tx2.lock().unwrap().send(s).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    // consumidor
    let consumer_handle = thread::spawn(move || {
        while let Ok(data) = rx.recv_timeout(timeout) {
            // timeout para no dejar al thread bloqueado
            println!("Consumidor -- Recibido {}", &data);
            thread::sleep(Duration::from_millis(30));
        }
    });

    let handles = [en_producer_handle, sp_producer_handle, consumer_handle];

    for handle in handles {
        handle.join().expect("The thread paniqueo.");
    }
}

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex, Arc};


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Mensaje 1"),
            String::from("Mensaje 2"),
            String::from("Mensaje 3"),
            String::from("Mensaje 4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let shared_rx = Arc::new(Mutex::new(rx));
    let shared_rx1 = shared_rx.clone();

    thread::spawn(move || {
        let received = shared_rx1.lock().unwrap().recv().unwrap();
        println!("Thread Consumidor 1 - Recibido: {}", received);
        let received = shared_rx1.lock().unwrap().recv().unwrap();
        println!("Thread Consumidor 1 - Recibido: {}", received);

    });

    let shared_rx2 = shared_rx.clone();
    thread::spawn(move || {
        let received = shared_rx2.lock().unwrap().recv().unwrap();
        println!("Thread Consumidor 2 - Recibido: {}", received);
        let received = shared_rx2.lock().unwrap().recv().unwrap();
        println!("Thread Consumidor 2 - Recibido: {}", received);

    });


    thread::sleep(Duration::from_secs(6));

}

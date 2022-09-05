use std::thread;
use std::time::Duration;

fn main() {
    let s = String::from("hola");
    
    let handle = thread::spawn(move || {
        for i in 1..50 {
            println!("Soy el hilo y estoy en la vuelta {}   -- {}!", i, s);
            thread::sleep(Duration::from_millis(1));
        }
    });

    

    for i in 1..5 {
        println!("Soy el hilo principal y ejecuto la vuelta {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    

}

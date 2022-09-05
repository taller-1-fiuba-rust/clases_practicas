use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let s = "ddd";
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for j in 1..10 {

        // let s_clone = s.clone();
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!(
                    "Soy el hilo {} y estoy en la vuelta {} --- {}!",
                    j, i, &s                );
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // println!("{}", s);

    for i in 1..5 {
        println!("Soy el hilo principal y ejecuto la vuelta {}", i);
        thread::sleep(Duration::from_millis(1000));
    }
    // handle.join().unwrap();
    // handles.into_iter().for_each(|h| h.join().unwrap());
    for handle in handles {
        handle.join().unwrap()
    }
    //thread::sleep(Duration::from_millis(1000));
}

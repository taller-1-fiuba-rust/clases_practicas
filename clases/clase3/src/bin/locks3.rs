use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // Aca hay mas de 1 problema :'(

    let lock = Arc::new(RwLock::new(0));
    let c_lock = lock.clone();
    
    {
    let valor = lock.read().unwrap();
    println!("valor:  {}", *valor);
    }
    thread::Builder::new().name("child thread".to_string()).spawn(move || {
        let _lock = c_lock.write().unwrap();
        panic!();
    }).unwrap().join().ok();
    
    
    println!("lock is poisoned?  {}", lock.is_poisoned());
}
use std::thread;
fn main() {
    let otro_hilo = thread::spawn(|| 
        thread::current().id()
    );

    let otro_hilo_id = otro_hilo.join().unwrap();
    println!("Id del hilo hijo: {:?}", otro_hilo_id);
}

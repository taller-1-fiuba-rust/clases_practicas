use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


fn main() {
    let vector = vec![1, 2, 3];
    let v = Arc::new(Mutex::new(vector));

    let shared_v = v.clone();

    let handle1 = thread::spawn(move || {
        let mut locked_vector = shared_v.lock().unwrap();
        locked_vector.push(4);

        println!("Here's a vector: {:?}", locked_vector);
    });

    let shared_v2 = v.clone();

    let handle2 = thread::spawn( move || {


        let mut locked_vector = shared_v2.lock().unwrap();
        locked_vector.push(10);
        println!("Here's a vector: {:?}", locked_vector);
    });
    drop(v);

    handle1.join().unwrap();
    handle2.join().unwrap();
}

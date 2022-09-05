
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {

    // Tengo un recurso que quiero compartir entre 2 o mas hilos.
    // Como por ejemplo un archivo.
    let file = File::create("file.txt")?;
    let file_with_lock = Arc::new(Mutex::new(file));
    let file_with_lock_ref = file_with_lock.clone();
    
    thread::Builder::new().name("thread-A".to_string()).spawn(move || {
        for _ in 1..1000 {
            let mut file_guard = file_with_lock_ref.lock().unwrap();
            let file_size = file_guard.metadata().unwrap().len();
            file_guard.seek(SeekFrom::Start(file_size)).unwrap();
            file_guard.write_all(b"A").unwrap();
        }

    })?.join().ok();

    thread::Builder::new().name("thread-B".to_string()).spawn(move || {
        for _ in 1..1000 {
            let mut file_guard = file_with_lock.lock().unwrap();
            let file_size = file_guard.metadata().unwrap().len();
            file_guard.seek(SeekFrom::Start(file_size)).unwrap();
            file_guard.write_all(b"B").unwrap();
        }

    })?.join().ok();



    Ok(())    
}
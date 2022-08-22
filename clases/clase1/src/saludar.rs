use std::io;

/// Pide lee una linea por stdin e imprime el nombre stdout
pub fn saludar() {
    let stdin = io::stdin();
    let mut nombre = String::new();
    println!("Ingrese su nombre");
    stdin.read_line(&mut nombre).unwrap();
    println!("Nombre: {:?}", nombre);
}

/// Imprime por stdout un saludo
pub fn saludar_nombre(nombre: &String) {
    println!("Hola {:?}", nombre);
}

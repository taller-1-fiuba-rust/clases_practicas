mod correlativas;
mod saludar;

use saludar::saludar_nombre;

fn main() {
    //saludar();
    let nombre = String::from("Pablo");

    saludar_nombre(&nombre);
}

//! Ubicación en memoria de las variables: stack y heap
use std::mem::size_of_val;

trait Persona {
    fn get_nombre(&self) -> String;
}

#[derive(Debug)]
struct Alumno {
    padron: u32,
    nombre: String,
}

#[derive(Debug)]
struct Docente {
    legajo: u32,
    nombre: String,
    regular: bool,
}

/// Represeta un alumno con padrón y nombre
impl Alumno {
    pub fn new(padron: u32, nombre: &str) -> Self {
        Self {
            padron,
            nombre: nombre.to_owned(),
        }
    }
}

/// Representa un docente con padrón, nombre y si es regular
impl Docente {
    pub fn new(legajo: u32, nombre: &str, regular: bool) -> Self {
        Self {
            legajo,
            nombre: nombre.to_owned(),
            regular,
        }
    }
}

impl Persona for Alumno {
    fn get_nombre(&self) -> String {
        self.nombre.clone()
    }
}

impl Persona for Docente {
    fn get_nombre(&self) -> String {
        self.nombre.clone()
    }
}
/// Imprime los tamaños en memoria de los distintos strings
/// Existen 2 tipos de strings: cadenas de caracteres `str` y objetos `String`
/// Un objeto `String` tiene un puntero a un segmento de memoria en el heap, además de la longitud de la cadena y longitud del segmento de memoria.
/// Un `str` es un *slice* de memoria. Puede hacer referencia a una porción de memoria correspondiente a un string,
/// o a cualquier porción de memoria accesible a la aplicación
/// Notar que un string literal vive en el data segment.
/// Por este motivo no podemos hacer que las cadenas del data segment sean mutables
/// Más información: https://blog.thoughtram.io/string-vs-str-in-rust/
fn print_string_size() {
    let nombre_str = "Ferris <3";
    // Equivale a String::new("Ferris");
    let mut nombre_owned = nombre_str.to_owned();
    println!("Tamaños de strings");
    println!(
        "size_of &str({:?}): {:?}",
        nombre_str,
        size_of_val(nombre_str)
    );
    println!(
        "size_of owned({:?}): {:?}",
        nombre_owned,
        size_of_val(&nombre_owned)
    );
    nombre_owned.replace_range(1..2, "oo");
    println!(
        "size_of owned cambiado({:?}): {:?}",
        nombre_owned,
        size_of_val(&nombre_owned)
    );
}

/// Imprime los tamaños en memoria de los distintos arrays y vectores.
/// Casi análogo a `print_string_size`, comparamos el tamaño en memoria de
/// los arrays y objetos de tipo vector.
fn print_vectors_size() {
    let nums_arr = [1, 3, 5, 7, 9];
    let mut nums_vec = vec![1, 3, 5, 7, 9];
    println!("Tamaños de vectores");
    println!("size_of({:?}): {:?}", nums_arr, size_of_val(&nums_arr));
    println!("size_of({:?}): {:?}", nums_vec, size_of_val(&nums_vec));

    nums_vec.insert(5, 11);

    println!("size_of({:?}): {:?}", nums_vec, size_of_val(&nums_vec));
}

/// Imprime el tamaño de un objeto *Alumno*, y luego lo copia al heap utilizando un `Box`
fn print_box_size() {
    let alumno = Alumno::new(99000, "Agus");
    println!("size_of({:?}): {:?}", alumno, size_of_val(&alumno));
    // La parte "genérica" del box puede ser deducida, por lo que Box::<Alumno>::new(alumno)
    // puede ser deducido desde Box::new(alumno)
    // ¡Notar que se está consumiendo el alumno! Ahora el *owner* del alumno es alumno_box
    let alumno_box = Box::new(alumno);
    // No compila porque `alumno` fue consumido
    // println!("size_of({:?}): {:?}", alumno, size_of_val(&alumno));

    // `alumno_box` es un puntero a una instancia del alumno, pero este alumno está en el heap
    println!("Tamaños de boxes");
    println!("size_of({:?}): {:?}", alumno_box, size_of_val(&alumno_box));
}

/// Movemos distintos objetos a boxes del mismo tipo
/// Tanto alumno como docente implementan el `trait Debug`
/// Por lo tanto puedo mover estos objetos en el heap y que el Box refiera a ellos por el *trait* que implementan
fn vector_of_boxes() {
    let otro_alumno = Alumno::new(99999, "Rafa");
    let docente = Docente::new(41213, "Fulano", false);

    // Si quiero crear un vector con un docente y un alumno, no me deja porque son de distinto tipo
    // let clase = vec![otro_alumno, docente];

    let otro_alumno_box: Box<dyn Persona> = Box::new(otro_alumno);
    let docente_box: Box<dyn Persona> = Box::new(docente);

    let clase = vec![otro_alumno_box, docente_box];
    // Como los elementos del vector implementan el trait Debug, puedo usar el print formateado
    println!("clase:");
    for persona in clase {
        println!("{}", persona.get_nombre());
    }
}

fn main() {
    print_string_size();
    print_vectors_size();
    print_box_size();
    vector_of_boxes();
}

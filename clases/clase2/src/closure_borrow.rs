//! Ejemplos de borrowing utilizando closures
#[derive(Debug)]
struct Alumno {
    padron: u32,
    nombre: String,
}

/// Ejemplos de Borrow
/// Declaramos 2 alumnos no mutables, a uno lo pasamos a un closure por movimiento
/// A otro alumno lo pasamos a un closure que lo toma por referencia, "prestado" (borrowed)
/// Finalmente, armamos un alumno mutable para modificar dentro del alumno
fn main() {
    let alumno_move = Alumno {
        padron: 91234,
        nombre: "Pedro".to_string(),
    };
    let alumno_ref = Alumno {
        padron: 91235,
        nombre: "Pablo".to_string(),
    };
    let mut alumno_ref_mut = Alumno {
        padron: 91235,
        nombre: "Matias".to_string(),
    };

    let funcion_move = |alumno: Alumno| {
        println!("Consumiendo a {:?}", alumno);
    };

    let funcion_ref = |alumno: &Alumno| {
        println!("Pasando por referencia a {:?}", alumno);
    };

    let funcion_ref_mut = |alumno: &mut Alumno| {
        alumno.nombre = alumno.nombre.clone() + " [mod]";
        println!("Pasando por referencia a {:?}", alumno);
    };

    // El alumno se "consume", el closure toma control de la vida del alumno
    // y la vida del mismo se limita al scope del closure
    funcion_move(alumno_move);
    //funcion_move(alumno_move);

    // Como lo paso por referencia, el alumno no se consume, entonces puedo reutilizarlo
    funcion_ref(&alumno_ref);
    funcion_ref(&alumno_ref);

    // Como alumno_ref es un alumno no mutable, lo siguiente fallaría
    // funcion_ref_mut(&mut alumno_ref);
    funcion_ref_mut(&mut alumno_ref_mut);
}

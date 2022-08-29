//! Ejemplo de lifetimes
//! Curso posee un nombre y una referencia a un docente.
//! Vemos como el indicador de lifetimes nos restringe a que las instancias de Curso deben
//! vivir el mismo o menos tiempo que la instancia de Docente.
#[derive(Debug)]
struct Curso<'a> {
    nombre: String,
    docente: &'a Docente,
}

#[derive(Debug)]
struct Docente {
    legajo: u32,
    nombre: String,
    regular: bool,
}

impl Docente {
    pub fn new(legajo: u32, nombre: &str, regular: bool) -> Self {
        Self {
            legajo,
            nombre: nombre.to_string(),
            regular,
        }
    }
}

impl<'a> Curso<'a> {
    pub fn new(nombre: &str, docente: &'a Docente) -> Self {
        Self {
            nombre: nombre.to_string(),
            docente,
        }
    }
}

fn main() {
    let docente = Docente::new(112233, "Pablo", true);
    let curso = Curso::new("taller", &docente);
    println!("curso: {:?}", curso);
}

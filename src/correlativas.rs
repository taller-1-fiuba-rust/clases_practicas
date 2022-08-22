//! # Correlativas
//! ejemplo de enumerados compuestos  
/// Representa una carrera que puede cursar la materia
#[derive(Debug)]
pub enum Carrera {
    Informatica,
    Licenciatura(u32),
    Electronica,
}

/// Imprime las correlativas según la carrera
pub fn print_correlativas(carrera: &Carrera) {
    match carrera {
        Carrera::Informatica => {
            println!("Informática: Estructura del computador y Análisis numérico")
        }
        Carrera::Licenciatura(1986) => {
            println!("Licenciatura('86): Algoritmos II y Organización del computador")
        }
        Carrera::Licenciatura(2014) => {
            println!("Licenciatura('14): Algoritmos III y Organización del computador")
        }
        Carrera::Electronica => {
            println!("Electrónica: Algoritmos II");
        }
        // Comentar para ver error por ramas del match no cubiertas
        _ => println!("Plan desconocido"),
    }
}

#[test]
fn test_correlativas() {
    print_correlativas(&Carrera::Informatica);
    print_correlativas(&Carrera::Licenciatura(1986));
    print_correlativas(&Carrera::Licenciatura(3000));
}

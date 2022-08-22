/// Tipo enumerado básico
/// "Extiende" `Debug` para poder imprimirse por pantalla
/// y `PartialEq` para poder usar el == en los tests
#[derive(Debug, PartialEq)]
pub enum Seniority {
    Trainee,
    Junior,
    SemiSenior,
    Senior,
}

/// Devuelve la "seniority" según la cantidad de años de experiencia
pub fn get_seniority(years: u32) -> Seniority {
    match years {
        0 => Seniority::Trainee,
        // Puedo hacer match con múltiples valores
        1 | 2 => Seniority::Junior,
        // Puedo utilizar un rango de valores (inclusivo)
        3..=5 => Seniority::SemiSenior,
        // Puedo usar un caso "default"
        // Los rangos semi-abiertos todavía no son estables (ej: "6..")
        _ => Seniority::Senior,
    }
}

#[test]
fn match_seniority() {
    assert_eq!(get_seniority(0), Seniority::Trainee);
    assert_eq!(get_seniority(2), Seniority::Junior);
    assert_eq!(get_seniority(5), Seniority::SemiSenior);
    assert_eq!(get_seniority(10), Seniority::Senior);
}

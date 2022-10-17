pub fn sumar(a: usize, b: usize) -> usize {
    a + b
}

pub fn dividir(a: usize, b: usize) -> Result<usize, String> {
    if b == 0 {
        return Err(String::from("No se puede dividir por cero"));
    }

    Ok(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumar_ok() {
        let result = sumar(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn dividir_ok() {
        let result = dividir(12, 2);
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn dividir_por_cero() {
        let result = dividir(12, 0);

        // si conozco el error exacto puedo hacer un assert
        assert_eq!(result, Err(String::from("No se puede dividir por cero")));

        // si no conozco el error exacto puedo usar un match, un if let o tambien:
        assert!(matches!(result, Err(_)));
    }

    // Una funcion de test tambien puede devolver un Result
    #[test]
    fn dividir_con_operator() -> Result<(), String> {
        // en caso de devolver Ok, el test continua sin problemas
        dividir(12, 3)?;
        dividir(24, 8)?;

        // caso de error, por ej: dividir por cero, se corta la ejecucion
        //dividir(12, 0)?;
        // el test toma cualquier Err devuelto como FAILED

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_espera_panic() {
        panic!("Este test debe tirar panic para que se considere correcto..")
    }

    #[test]
    #[ignore]
    fn test_ignorado() {
        panic!("Este test esta ignorado por eso 'cargo test' no falla..")
    }
}

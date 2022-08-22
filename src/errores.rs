/// Errores propios del dominio que pueden ocurrir al dividir
/// (por fines didácticos, si se divide y el resultado no es entero, es un error)
#[derive(Debug)]
pub enum ArithError {
    DivBy0,
    ResultNotInteger,
}

/// Funcion que devuelve tipo std::result::Result
/// Este enumerado tiene 2 valores: Ok<T1>, o Err<T2>
/// Si quiero indicar que la función fue exitosa, devuelvo el resultado esperado dentro de Ok()
/// Si quiero indicar que falló, devuelvo el error dentro de Err()
/// Si quiero devolver "void", puedo hacer, por ejemplo Ok(())
pub fn dividir_enteros(dividendo: i32, divisor: i32) -> Result<i32, ArithError> {
    if divisor == 0 {
        Err(ArithError::DivBy0)
    } else {
        if dividendo % divisor != 0 {
            Err(ArithError::ResultNotInteger)
        } else {
            Ok(dividendo / divisor)
        }
    }
}

/// Uso del operador "?""
/// Si uno de los "dividir_enteros" falla, el error es devuelto inmediatamente
pub fn son_iguales_rebuscado(a: i32, b: i32) -> Result<(), ArithError> {
    dividir_enteros(a, b)?;
    dividir_enteros(b, a)?;
    Ok(())
}

#[test]
pub fn test_dividir() {
    // dividir_enteros(10, 2) -> Ok(5)
    // assert_eq!(dividir_enteros(10, 2), Ok(5))
    // dividir_enteros(10, 2).unwrap() -> extraigo 5 del Ok(5)
    assert_eq!(dividir_enteros(10, 2).unwrap(), 5);
    // dividir_enteros(10, 3) -> Err()
    assert!(dividir_enteros(10, 3).is_err());
    son_iguales_rebuscado(4, 4).unwrap();
    assert!(son_iguales_rebuscado(4, 8).is_err());
}

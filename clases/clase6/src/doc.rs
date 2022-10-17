/// La primer linea representa una descripcion breve de nuestra funcion.
///
/// Las siguientes lineas son la documentacion detallada. El bloque de codigo comienza
/// con triple ` y tiene un `fn main()` y un `extern crate <cratename>` implicito.
///
/// ```
/// let result = clase_testing::doc::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Generalmente los comentarios de documentacion incluyen "Examples", "Panics" y "Failures".
///
/// La siguiente funcion divide dos numeros.
///
/// # Examples
///
/// ```
/// let result = clase_testing::doc::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// La funcion da panic si el segundo argumento es cero.
///
/// ```should_panic
/// // panic al dividir por cero
/// clase_testing::doc::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("No se puede dividir por cero");
    }

    a / b
}

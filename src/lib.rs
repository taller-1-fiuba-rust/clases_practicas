//! # Intro a Rust
//! Los siguientes son módulos con pequeños ejemplos de sintaxis de Rust
//!
//! * módulo [saludar](saludar):
//! En este modulo vamos a ver como definir funciones sencillas.
//! * módulo [sort](sort):
//! En este modulo vamos a encontrar una funcion sencilla para ordenar un\
//! vector de numeros.
//! * módulo [iteraciones](iteraciones):
//! en este ejemplo vemos distintas formas de iterar sobre una colección
//! Desde la forma clásica usando un índice, al uso de iteradores.
//! * módulo [seniority](seniority):
//! Ejemplo de un enum sencillo, similar a los de otros lenguajes,
//! y de bloque `match` para rangos.
//! * módulo [correlativas](correlativas):
//! Ejemplo de un enum compuesto, y de cómo el bloque `match` se usa para los mismos.
//! * módulo [options](options):
//! Ejemplo del enum Option, cómo es que podemos usarlo para indicar que
//! una función devuelve o "algo válido" envuelto en un enumerado de tipo `Some`
//! o "nada", como valor `None.
//! * módulo [figuras](figuras):
//! En este modulo encontramos un tipo enumerado sencillo para las
//! figuras geometricas. Tambien definimos un metodo que aplica a todas
//! ellas.
//! * módulo [errores](errores):
//! De forma similar al enum Option, podemos usar el enum Result para obtener
//! un dato válido, de tipo `Ok` si una función se ejecutó exitosamente
//! o un resultado de tipo `Err` si ocurrió un error dentro de la función
//! también se muestra como propagar los errores utilizando el operador `?`.
//! * módulo [coordenada](coordenada):
//! Ejemplo de TDA `Coordenada`. En este ejemplo podemos ver como crear una instancia
//! de una estructura usando un método de clase y cómo usar métodos asociados al struct
//! * módulo [builder_helados](builder_helados):
//! En este ejemplo tenemos 2 TDAs: `Helado` y `HeladoBuilder`. Implementamos el patrón
//! *Builder* para personalizar la creación de un objeto `Helado`

/// Patrón builder
pub mod builder_helados;
/// TDA coordenada
pub mod coordenada;
/// Ejemplo de enums (2)
pub mod correlativas;
/// Ejemplo de manejo de errores
pub mod errores;
/// Ejemplo de iteraciones en una colección
pub mod iteraciones;
/// Ejemplo de enum Option
pub mod options;
/// Ejemplo de enums
pub mod seniority;
/// Ejemplo sort
pub mod sort;
// Ejemplo figuras geometricas
pub mod figuras;
/// Definicion de funciones sencillas
pub mod saludar;

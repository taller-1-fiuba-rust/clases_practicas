use crate::saludar::saludar_nombre;

#[test]
pub fn iterar() {
    let lista_nombres = vec![String::from("Don Pepito"), String::from("Don Jose")];

    // 3 formas de iterar:

    // Forma 1: Usando el largo de la lista.
    for i in 0..lista_nombres.len() {
        saludar_nombre(&lista_nombres[i]);
    }

    // Forma 2: Usando for __ in ___
    // Ojo, usamos el & para no consumir la lista.
    // Lo vamos a ver en detalle en la segunda clase.
    // (internamente hay un into_iter())
    for nombre in &lista_nombres {
        saludar_nombre(nombre);
    }

    // Forma 3: Usando iter() y foreach( closure )
    lista_nombres.iter().for_each(|nombre| {
        saludar_nombre(nombre);
    });

    // Equivalente a la de arriba.
    // Nos ahorramos hacer un closure.
    lista_nombres.iter().for_each(saludar_nombre);
}

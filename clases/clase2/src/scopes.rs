use std::collections::HashMap;

#[derive(Debug)]
struct Contador {
    clave: String,
    valor: u32,
}

impl Contador {
    pub fn new(clave: String) -> Self {
        Contador { clave, valor: 0 }
    }

    fn imprimir(&self) {
        println!("El valor actual es: {:?}", self);
    }

    fn sumar(&mut self) {
        self.valor = self.valor + 1;
    }
}

fn main() {
    let contador_const = Contador::new("const".to_owned());
    contador_const.imprimir();
    //contador_const.sumar();

    let mut contador_mut = Contador::new("mut".to_owned());
    contador_mut.sumar();
    contador_mut.imprimir();

    //borrow_vector();
    //borrow_vector_mut();
    borrow_dict_mut();
    partial_move();
}

fn borrow_vector() {
    let mi_vec = vec![
        "pepa".to_owned(),
        "pepe".to_owned(),
        "pepi".to_owned(),
        "pepo".to_owned(),
    ];
    // No tenemos problemas en pedir múltiples referencias porque son "read only"
    let valor = &mi_vec[0];
    let mismo_valor = &mi_vec[0];
    println!("{:?}", valor);
    println!("{:?}", mismo_valor);
    println!("{:?}", mi_vec.len());
}

fn borrow_vector_mut() {
    let mut mi_vec = vec![
        "pepa".to_owned(),
        "pepe".to_owned(),
        "pepi".to_owned(),
        "pepo".to_owned(),
    ];
    //let primero = &mi_vec[0];

    let primero = &mut mi_vec[0];
    primero.push_str(" pig");

    println!("Valor: {:?}", primero);
    // En este momento `primero` es una referencia a un elemento de mi_vec
    // La siguiente operación, que sería pasarle a la función `push` una referencia mutable "self"
    // es válida porque `primero` no se usa más.
    mi_vec.push("pepu".to_owned());
    // PERO, si descomento la siguiente linea, estoy haciendo uso de una referencia al vector luego
    // de haberlo modificado. Dependiendo si `primero` es mutable o no, el mensaje de error será
    // distinto.
    //println!("primero: {:?}", primero);
}

/// En rust 1.55 el siguiente código genera un warning
fn borrow_dict_mut() {
    let mut mi_dict = HashMap::<String, usize>::new();
    mi_dict.insert("prueba".to_owned(), 0);
    let prueba = mi_dict.get("prueba");
    match prueba {
        Some(valor) => mi_dict.insert("prueba".to_owned(), valor + 1),
        None => mi_dict.insert("prueba".to_owned(), 0),
    };

    // Comparar la forma anterior con la siguiente
    if let Some(valor) = mi_dict.get_mut("prueba2") {
        *valor += 1;
    } else {
        mi_dict.insert("prueba".to_owned(), 0);
    }

    // Analizar la siguiente linea
    // let valor_prueba2 = mi_dict.entry("prueba2".to_string()).or_insert(0);
}

fn partial_move() {
    let mut contador_mut = Contador {
        valor: 0,
        clave: "borrow".to_owned(),
    };
    // Esto ocasiona un movimiento parcial
    //let nombre_contador = contador_mut.clave;
    // Una referencia no
    let nombre_contador_ref = &contador_mut.clave;
    contador_mut.imprimir();
    let m = contador_mut.valor + 3;
    // No puedo usar el valor mientras esté tomado
    contador_mut.clave = contador_mut.clave + " [mod]";
    contador_mut.imprimir();
    // Tampoco puedo usar la referencia luego de ser tomada
    //nombre_contador_ref.len();
}

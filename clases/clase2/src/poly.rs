//! Ejemplos de polimorfismo
#[derive(Debug)]
struct Perro {
    nombre: String,
}

fn imprimir<T>(algo: T) where T: std::fmt::Debug + Clone {
    println!("{:?}", algo)
}
#[derive(Debug)]
struct Vaca {
    nombre: String,
}

// Exijo que el trait PuedeHablar se aplique sobre algo con el trait std::fmt::Debug
trait PuedeHablar: std::fmt::Debug {
    fn hablar(&self);
}

impl PuedeHablar for Perro {
    fn hablar(&self) {
        println!("{} dice 'guau'", self.nombre);
    }
}

impl PuedeHablar for Vaca {
    fn hablar(&self) {
        println!("{} dice 'mu'", self.nombre);
    }
}

fn hacer_hablar<T: PuedeHablar>(animales: Vec<T>) {
    for animal in animales {
        animal.hablar()
    }
}

fn hacer_hablar_dinamico(animales: Vec<Box<dyn PuedeHablar>>) {
    for animal in animales {
        animal.hablar()
    }
}

fn main() {
    println!("Polimorfismo estático");
    let lola = Vaca {
        nombre: "Lola".to_string(),
    };
    let negro = Perro {
        nombre: "Negro".to_string(),
    };
    presentar(&lola);
    presentar(&negro);

    println!("Polimorfismo dinámico");
    let animales: Vec<Box<dyn PuedeHablar>> = vec![
        Box::new(Vaca {
            nombre: "Violeta".to_string(),
        }),
        Box::new(Perro {
            nombre: "Pancho".to_string(),
        }),
    ];

    for animal in &animales {
        animal.hablar();
    }

    // hacer_hablar_dinamico(animales);
    // hacer_hablar(animales);

    let vector_de_vacas = vec![
        Vaca {
            nombre: "Lola".to_string(),
        },
        Vaca {
            nombre: "La vaca de Milka".to_string(),
        },
    ];

    // hacer_hablar_dinamico(vector_de_vacas);
    // hacer_hablar(vector_de_vacas);
}

// `Presentar()` es polimórfico en *tiempo de compilación*
// Al momento de compilar ya se sabe qué tipo de variable va a recibir la función

fn presentar(animal: &(dyn PuedeHablar)) {
    println!("Se presenta {:?}", animal);
    animal.hablar();
}

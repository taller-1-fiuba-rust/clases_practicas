#[allow(dead_code)]
#[derive(Debug)]
enum Recipiente {
    Copita,
    Cucurucho,
    Vaso,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Gusto {
    Vainilla,
    DulceDeLeche,
    Chocolate,
    Frutilla,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Helado {
    recipiente: Recipiente,
    gustos: Vec<Gusto>,
    cucharita: bool,
}

impl Helado {
    /// Constructor de helado
    /// Este método "new" recibe los atributos necesarios para armar el helado.
    /// Noten que Rust nos exige que inicialicemos todos los valores de un objeto.
    /// Ni tampoco podemos tener parametros con valores por default (como por ejemplo en Python).
    /// Por este motivo, el patrón Builder nos puede ayudar si el objeto es complejo
    /// de construir, permitiéndonos asignar los atributos por etapas.
    pub fn new(recipiente: Recipiente, gustos: Vec<Gusto>, cucharita: bool) -> Self {
        Helado {
            recipiente,
            gustos,
            cucharita,
        }
    }
}

/// Builder de helados. Esta estructura tiene los mismos atributos que Helado
#[derive(Debug)]
struct HeladoBuilder {
    recipiente: Recipiente,
    gustos: Vec<Gusto>,
    cucharita: bool,
}

impl HeladoBuilder {
    /// El builder tiene un constructor default sin parámetros. Esto nos permite armar un
    /// prototipo de helado "vacío" e ir personalizándolo por etapas.
    pub fn new() -> Self {
        HeladoBuilder {
            recipiente: Recipiente::Vaso,
            gustos: vec![],
            cucharita: false,
        }
    }

    /// Devuelve una instancia de `Helado`
    /// Por un lado, notar que el método "consume" a self
    /// Esto es para que no se reutilice el builder para construir otro helado igual
    /// Por otro lado, notar que se está devolviendo una instancia de Helado, pero
    /// bien se podría haber devuelto un Result, de forma tal que se puedan
    /// ejecutar validaciones, como por ejemplo, que haya mínimo un gusto
    ///
    /// ¡Recordar que los patrones se adaptan al problema, y no el problema a los patrones!
    pub fn build(self) -> Helado {
        Helado::new(self.recipiente, self.gustos, self.cucharita)
    }

    pub fn with_recipiente(mut self, recipiente: Recipiente) -> Self {
        self.recipiente = recipiente;
        self
    }

    pub fn with_cucharita(mut self, cucharita: bool) -> Self {
        self.cucharita = cucharita;
        self
    }

    pub fn add_gusto(mut self, gusto: Gusto) -> Self {
        self.gustos.push(gusto);
        self
    }
}

#[test]
fn test_builder_basico() {
    let builder = HeladoBuilder::new().with_cucharita(true);
    println!("Helado con cucharita: {:?}", builder.build());
}

#[test]
fn test_builder_complejo() {
    let builder = HeladoBuilder::new()
        .with_recipiente(Recipiente::Cucurucho)
        .add_gusto(Gusto::DulceDeLeche)
        .add_gusto(Gusto::Chocolate);
    println!("Helado con 2 gustos: {:?}", builder.build());
}

/// #TDA Coordenada
/// Representa una coordenada en el plano 2d
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    /// Constructor
    /// Devolver Self es equivalente a devolver la clase que estoy "extendiendo"
    pub fn new(x: i32, y: i32) -> Self {
        // Lo siguiente es equivalente a
        // return Coord {x: x, y: y};
        // Debido a que las variables tienen el mismo nombre que los atributos del struct
        Coord { x, y }
    }

    /// Devuelve el módulo del vector representado por la coordenada
    pub fn modulo(&self) -> f32 {
        let cuadrado = (self.x * self.x) + (self.y * self.y);
        (cuadrado as f32).sqrt()
    }
}

#[test]
pub fn test_coord() {
    let coordenada = Coord::new(6, 8);
    let modulo = coordenada.modulo();
    assert_eq!(modulo, 10.0);
}

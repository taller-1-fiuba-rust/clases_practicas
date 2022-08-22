use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum Figura {
    Cuadrado(f64),
    Rectangulo { base: f64, altura: f64 },
    Circulo { radio: f64 },
}

impl Figura {
    pub fn area(&self) -> f64 {
        match &self {
            Figura::Cuadrado(lado) => lado * lado,
            Figura::Rectangulo { base, altura } => base * altura,
            Figura::Circulo { radio } => PI * radio.powi(2),
        }
    }
}

#[test]
fn test_area() {
    let radio = 3.0;
    assert_eq!(9., Figura::Cuadrado(3.).area());
    assert_eq!(
        6.0,
        Figura::Rectangulo {
            base: 2.0,
            altura: 3.0
        }
        .area()
    );
    assert_eq!(PI * radio * radio, Figura::Circulo { radio: 3.0 }.area());
}

#[test]
fn test_equality() {
    assert_eq!(
        Figura::Circulo { radio: 3.0 },
        Figura::Circulo { radio: 3.0 }
    );
}

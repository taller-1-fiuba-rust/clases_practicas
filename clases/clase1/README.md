# Práctica - Primer Clase

## Descargar compilador

<https://www.rust-lang.org/learn/get-started> -> instalación default

Ejecutar `source $HOME/.cargo/env` para poder empezar

## Cargo

**Cargo** es el manejador de paquetes de Rust. Posee varios comandos para compilar y ejecutar nuestros proyectos, además de gestionar paquetes de software externo.

Algunos comandos útiles:

`cargo new <nombre>`: crea una carpeta con el nombre `<nombre>`. Dentro de la misma se encuentra un "hola mundo" y un repositorio git.

`cargo build`: compila nuestro proyecto

`cargo run`: ejecuta el proyecto, lo compila si es necesario

`cargo test`: ejecuta los tests presentes en el proyecto.

`cargo test -- --nocapture`: ejecuta tests, pero mostrando las escrituras a stdout (es decir, muestra los `println`)

`cargo doc`: compila la documentación del proyecto. `cargo doc --open` la abre en un navegador

Información relevante al proyecto puede ser configurada en el archivo `cargo.toml`

> Dato curioso: para el proceso de linkeo, cargo necesita un linker de C, así que recordar instalarlo con `sudo apt install build-essential`

## Rustup

`rustup` es nuestro gestor de *toolchains*

Un *toolchain* es un conjunto de herramientas utilizados en el proceso de compilación.

Cuando instalamos Rust, instalamos el toolchain "estable". Sin embargo, algunos *features* del lenguaje están disponible sólamente en el toolchain inestable o "nightly".

Algunos comando útiles son:

* `rustup update` actualizar toolchain y componentes
* `rustup default` -> `rustup default stable|nightly|<nombre>` elegir toolchain
* `rustup component list`: componentes que pertenecen al toolchain

## Ejemplos de sintaxis

Compilar utilizando `cargo build`

Generar y abrir la documentación utilizando `cargo doc --open`

## Bibliografia Relacionada

* [Rust Book: Capitulo 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
* [Rust Book: Capitulo 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
* [Rust Book: Capitulo 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
* Rust in Action: Capitulos 2 y 3.

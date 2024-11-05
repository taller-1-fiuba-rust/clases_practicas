use chat::{client::run_client, error::ChatError};

const REQUIRED_ARGS: usize = 2;

fn main() -> Result<(), ChatError> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != REQUIRED_ARGS {
        println!("Cantidad de argumentos inválido");
        println!("Usage:\n{} <host:puerto>", &argv[0]);
        return Err(ChatError::InvalidParameters);
    }

    run_client(&argv[1])
}

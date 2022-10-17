use std::{error::Error, path::PathBuf};

use clase_testing::files;

#[test]
fn int_files_ok() -> Result<(), Box<dyn Error>> {
    // GIVEN: un archivo de configuracion valido
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/testfile.txt");

    // WHEN: se ejecuta la funcion from_file de Config
    let cfg = files::Config::from_file(path.to_str().unwrap())?;

    // THEN: los valores del archivo de configuracion se levantan en el struct Config
    assert_eq!(cfg.tcp_port, 44444);
    assert_eq!(cfg.log_directory, "/tmp/test");
    assert_eq!(cfg.max_log_file_kb_size, 65536);
    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

pub struct Config {
    pub tcp_port: u16,
    pub log_directory: String,
    pub max_log_file_kb_size: u32,
}

impl Config {
    /// Crea un config leyendo un archivo de configuracion ubicado en la ruta especificada por parametro.
    /// El formato del contenido es: {config_name}={config_value}
    /// Devuelve un Config con los valores leidos del archivo especificado
    ///
    /// Devuelve un io::Error si:
    /// - No se pudo encontrar el archivo en la ruta indicada.
    /// - El archivo tiene un formato invalido.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        Self::from_reader(file)
    }

    /// Crea un config a partir de cualquier implementacion del trait Read
    /// Ver implementaciones en: https://doc.rust-lang.org/std/io/trait.Read.html#implementors
    fn from_reader<T: Read>(content: T) -> Result<Config, Box<dyn Error>> {
        let reader = BufReader::new(content);

        let mut cfg = Self {
            tcp_port: 0,
            log_directory: String::new(),
            max_log_file_kb_size: 0,
        };
        for line in reader.lines() {
            let current_line = line?;
            let setting: Vec<&str> = current_line.split('=').collect();

            if setting.len() != 2 {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid config input: {}", current_line),
                )));
            }
            Self::load_setting(&mut cfg, setting[0], setting[1])?;
        }
        Ok(cfg)
    }

    fn load_setting(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match name {
            "LOG_DIRECTORY" => self.log_directory = String::from(value),
            "TCP_PORT" => self.tcp_port = u16::from_str(value)?,
            "MAX_LOG_FILE_KB_SIZE" => self.max_log_file_kb_size = u32::from_str(value)?,
            _ => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid config setting name: {}", name),
                )))
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_con_formato_invalido() {
        // GIVEN: un reader con contenido invalido para el archivo de configuracion
        let content = "Hola Mundo!".as_bytes();

        // WHEN: se ejecuta la funcion from_reader con ese reader
        let cfg = Config::from_reader(content);

        // THEN: la funcion devuelve un Err porque el contenido es invalido
        assert!(cfg.is_err());
        assert!(matches!(cfg, Err(_)));
    }

    #[test]
    fn config_sin_valores_requeridos() -> Result<(), Box<dyn Error>> {
        // GIVEN: un reader con contenido de configuracion completo
        let content = "LOG_DIRECTORY=/tmp/test\n\
            TCP_PORT=9876\n\
            MAX_LOG_FILE_KB_SIZE=65536"
            .as_bytes();

        // WHEN: se ejecuta la funcion from_reader con ese reader
        let cfg = Config::from_reader(content)?;

        // THEN: la funcion devuelve Ok y los parametros de configuracion tienen los valores esperados
        assert_eq!(9876, cfg.tcp_port);
        assert_eq!("/tmp/test", cfg.log_directory);
        assert_eq!(65536, cfg.max_log_file_kb_size);
        Ok(())
    }
}

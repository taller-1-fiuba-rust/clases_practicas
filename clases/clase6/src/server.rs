use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    pub address: String,
}

impl Server {
    /// run inicia el server escuchando en la ip y puerto especificados en server address
    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address.clone())?;

        let connection = listener.accept()?;
        let client_stream: TcpStream = connection.0;

        self.handle_handshake(client_stream)?;
        Ok(())
    }

    /// handle_handshake recibe un tcp stream y realiza el handshake con el cliente
    pub fn handle_handshake(&self, mut stream: TcpStream) -> io::Result<()> {
        self.handle_handshake_internal(&mut stream)
    }

    /// handle_handshake_internal recibe una referencia mutable que implementa los traits Read y Write
    /// y realiza el handshake con el cliente
    fn handle_handshake_internal<T: Read + Write>(&self, mut stream: T) -> io::Result<()> {
        let mut line = String::new();
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut line)?;

        if !line.eq("Handshake initiated!\n") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid handshake!\n",
            ));
        }

        let response_handshake = "Handshake completed!\n".as_bytes();
        stream.write(&response_handshake)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// MockTcpStream es una mock que implementa los traits Read y Write, los mismos que implementa el TcpStream
    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        /// Lee bytes del stream hasta completar el buffer y devuelve cuantos bytes fueron leidos
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.read_data.as_slice().read(buf)
        }
    }

    impl Write for MockTcpStream {
        /// Escribe el valor del buffer en el stream y devuelve cuantos bytes fueron escritos
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_data.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.write_data.flush()
        }
    }

    #[test]
    fn server_handshake_ok() -> io::Result<()> {
        // GIVEN: una instancia de server
        let server = Server {
            address: "0.0.0.0:9876".to_string(),
        };

        // AND: un mock de TcpStream que va a devolver un mensaje de handshake cuando se ejecute el metodo read
        let mut mock = MockTcpStream {
            read_data: "Handshake initiated!\n".as_bytes().to_vec(),
            write_data: Vec::new(),
        };

        // WHEN: el server procesa el mensaje del mock de stream
        server.handle_handshake_internal(&mut mock)?;

        // THEN: el server escribe la respuesta del handshake en el mock de stream
        assert_eq!(
            "Handshake completed!\n".as_bytes(),
            mock.write_data.as_slice()
        );
        Ok(())
    }

    #[test]
    fn server_handshake_fail() -> io::Result<()> {
        // GIVEN: una instancia de server
        let server = Server {
            address: "0.0.0.0:9876".to_string(),
        };

        // AND: un mock de TcpStream que va a devolver un mensaje de handshake cuando se ejecute el metodo read
        let mut mock = MockTcpStream {
            read_data: "Handshake with invalid format\n".as_bytes().to_vec(),
            write_data: Vec::new(),
        };

        // WHEN: el server procesa el mensaje del mock de stream
        let result = server.handle_handshake_internal(&mut mock);

        // THEN: el server devuelve un resultado Err porque el handshake no se pudo completar
        assert!(matches!(result, Err(e) if e.kind() == io::ErrorKind::InvalidData));
        Ok(())
    }
}

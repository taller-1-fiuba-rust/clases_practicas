use std::io::{Read, Write};

#[derive(Debug)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn read_from(stream: &mut dyn Read) -> std::io::Result<Message> {
        // Leo 4 bytes con la longitud del mensaje
        let mut num_buffer = [0u8; 4];
        stream.read_exact(&mut num_buffer)?;
        // Una vez que leemos los bytes, los convertimos a un u32
        let size = u32::from_be_bytes(num_buffer);

        // Creamos un buffer para el mensaje
        let mut msg_buf = vec![0; size as usize];
        stream.read_exact(&mut msg_buf)?;

        // Convierto de bytes a string.
        let content = String::from_utf8(msg_buf)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
        Ok(Message { content })
    }

    pub fn write_to(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        // Debo convertir una variable u32 a una cadena de bytes big_endian
        // Es importante aclarar el tipo de variable de len(), sino será usize
        let size_be = (self.content.len() as u32).to_be_bytes();
        stream.write(&size_be)?;
        stream.write(&self.content.as_bytes())?;
        Ok(())
    }
}

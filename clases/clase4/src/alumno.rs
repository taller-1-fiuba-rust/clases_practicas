use std::io::{Read, Write};

#[derive(Debug)]
pub struct Alumno {
    pub nombre: String,
    pub padron: u32
}

impl Alumno {
    pub fn write_to(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        // Debo convertir una variable u32 a una cadena de bytes big_endian
        // En algunos lenguajes se convierte a big-endian con la funcion ntohl/ntohs
        let padron_be = self.padron.to_be_bytes();
        stream.write(&padron_be)?;
        // Es importante aclarar el tipo de variable de len(), sino será usize
        let size_be = (self.nombre.len() as u32).to_be_bytes();
        stream.write(&size_be)?;
        stream.write(&self.nombre.as_bytes())?;
        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> std::io::Result<Alumno> {
        let mut num_buffer = [0u8; 4];
        // Leo 4 bytes, con el padrón
        stream.read_exact(&mut num_buffer)?;
        let padron = u32::from_be_bytes(num_buffer);
        // Vuelvo a leer 4 bytes con la longitud del nombre
        stream.read_exact(&mut num_buffer)?;
        // Una vez que leemos los bytes, los convertimos a un u32
        let size = u32::from_be_bytes(num_buffer);
        // Creamos un buffer para el nombre
        let mut nombre_buf = vec![0; size as usize];
        stream.read_exact(&mut nombre_buf)?;
        // Convierto de bytes a string.
        let nombre_str = std::str::from_utf8(&nombre_buf).expect("Error al leer nombre");
        let nombre = nombre_str.to_owned();
        let alumno = Alumno{nombre, padron};
        Ok(alumno)
    }
}

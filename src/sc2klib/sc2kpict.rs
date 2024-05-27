use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use super::sc2kfile::SC2KFileChunk;

#[derive(Debug)]
pub struct SC2KPictRow {
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct SC2KPict {
    pub header: Vec<u8>,
    pub x_size: u16,
    pub y_size: u16,
    pub rows: Vec<SC2KPictRow>,
}

impl SC2KPict {
    /// Extracts the picture data from a chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - PICT chunk from a SimCity 2000 city file.
    pub fn extract_data(&mut self, chunk: &SC2KFileChunk) -> io::Result<()> {
        let data = chunk.data.clone();
        self.header = data[0..4].to_vec();
        let mut current = &data[5..8];
        self.x_size = current.read_u16::<BigEndian>()?;
        self.y_size = current.read_u16::<BigEndian>()?;
        self.rows = Vec::new();

        Ok(())
    }
}

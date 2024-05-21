use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::sck2data::*;

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
    pub fn extract_data(chunk: &SC2KChunk) -> io::Result<SC2KPict> {
        let data = chunk.data.clone();
        let header = data[0..4].to_vec();
        let mut current = &data[5..8];
        let x_size = current.read_u16::<BigEndian>()?;
        let y_size = current.read_u16::<BigEndian>()?;
        let rows: Vec<SC2KPictRow> = Vec::new();

        let pict = SC2KPict {
            header,
            x_size,
            y_size,
            rows
        };

        Ok(pict)
    }
}
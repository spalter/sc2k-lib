use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::SC2KChunk;

#[derive(Debug, Default)]
pub struct SC2KCity {
    pub name: String,
    pub header: u32,
    pub mode: u32,
    pub rotation: u32,
    pub year_founded: u32,
    pub age: u32,
    pub money: u32,
    pub bonds: u32,
    pub level: u32,
    pub status: u32,
}

impl SC2KCity {
    pub fn extract_misc_data(name: String, chunk: &SC2KChunk) -> io::Result<SC2KCity>{
        let mut city = SC2KCity::default();
        let data = chunk.data.clone();
        let mut current = &data[0..data.len()];
        city.name = name;
        city.header = current.read_u32::<BigEndian>()?;
        city.mode = current.read_u32::<BigEndian>()?;
        city.rotation = current.read_u32::<BigEndian>()?;
        city.year_founded = current.read_u32::<BigEndian>()?;
        city.age = current.read_u32::<BigEndian>()?;
        city.money = current.read_u32::<BigEndian>()?;
        city.bonds = current.read_u32::<BigEndian>()?;
        city.level = current.read_u32::<BigEndian>()?;
        city.status = current.read_u32::<BigEndian>()?;

        Ok(city)
    }
}
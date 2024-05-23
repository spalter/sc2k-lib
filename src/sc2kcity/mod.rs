use std::io::{self};

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
    pub city_value: u32,
    pub land_value: u32,
    pub crime_count: u32,
    pub traffic_count: u32,
    pub pollution: u32,
    pub city_fame: u32,
    pub advertising: u32,
    pub garbage: u32,
    pub work_force_percent: u32,
}

impl SC2KCity {
    pub fn extract_misc_data(city: &mut SC2KCity, chunk: &SC2KChunk) -> io::Result<()> {
        let data = chunk.data.clone();
        let mut current = &data[0..data.len()];

        city.header = current.read_u32::<BigEndian>()?;
        city.mode = current.read_u32::<BigEndian>()?;
        city.rotation = current.read_u32::<BigEndian>()?;
        city.year_founded = current.read_u32::<BigEndian>()?;
        city.age = current.read_u32::<BigEndian>()?;
        city.money = current.read_u32::<BigEndian>()?;
        city.bonds = current.read_u32::<BigEndian>()?;
        city.level = current.read_u32::<BigEndian>()?;
        city.status = current.read_u32::<BigEndian>()?;
        city.city_value = current.read_u32::<BigEndian>()?;
        city.land_value = current.read_u32::<BigEndian>()?;
        city.crime_count = current.read_u32::<BigEndian>()?;
        city.traffic_count = current.read_u32::<BigEndian>()?;
        city.pollution = current.read_u32::<BigEndian>()?;
        city.city_fame = current.read_u32::<BigEndian>()?;
        city.advertising = current.read_u32::<BigEndian>()?;
        city.garbage = current.read_u32::<BigEndian>()?;
        city.work_force_percent = current.read_u32::<BigEndian>()?;

        Ok(())
    }
}
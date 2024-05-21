use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read};
use std::{collections::HashMap, fs::File};

use crate::sc2kcity::*;
use crate::sc2kpict::*;

#[derive(Debug)]
pub struct SC2KChunk {
    pub id: String,
    pub length: u32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SC2KCityData {
    pub file_type: u32,
    pub length: u32,
    pub container: u32,
    pub chunks: HashMap<String, SC2KChunk>,
    pub city: SC2KCity,
    pub pict: SC2KPict,
}

impl SC2KCityData {
    pub fn decompress(data: &[u8]) -> Vec<u8> {
        let buffer = Vec::new();

        if data[0] < 128 {
            print!("Uncompressed");
        } else if data[0] > 128 {
            print!("Compressed");
        }

        buffer
    }

    /// Extracts the city name from CNAM chunk.
    pub fn extract_city_data(&mut self) {
        match self.chunks.get("CNAM") {
            Some(chunk) => {
                let mut name = String::from("");
                let data = chunk.data.clone();
                for c in data {
                    match c {
                        31 => continue,
                        0 => break,
                        _ => name = format!("{}{}", name, c as char),
                    }
                }
                self.city.name = name.clone();
            }
            None => {}
        }

        match self.chunks.get("MISC") {
            Some(chunk) => {
                self.city = SC2KCity::extract_misc_data(self.city.name.clone(), &chunk).unwrap();
            }
            None => {}
        }
    }

    /// Extracts the picture from a PICT chunk.
    pub fn extract_pict_data(&mut self) {
        match self.chunks.get("PICT") {
            Some(chunk) => {
                self.pict = SC2KPict::extract_data(&chunk).unwrap();
            }
            None => {}
        }
    }

    /// Returns the SimCity 2000 City Data
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string containing the path to the city file.
    pub fn read_sc2k_city_file(file_path: &str) -> io::Result<SC2KCityData> {
        let mut file = File::open(file_path)?;
        let file_type = file.read_u32::<BigEndian>()?;
        let length = file.read_u32::<BigEndian>()?;
        let container = file.read_u32::<BigEndian>()?;
        let mut chunks: HashMap<String, SC2KChunk> = HashMap::new();
        let mut cursor = 0;

        // Extract all chunks
        while cursor < length - 4 {
            // Reduce by container close
            // Get the ASII ID
            let c1 = file.read_u8()?;
            let c2 = file.read_u8()?;
            let c3 = file.read_u8()?;
            let c4 = file.read_u8()?;
            let id = format!("{}{}{}{}", c1 as char, c2 as char, c3 as char, c4 as char);

            // Get the chunk size
            let size = file.read_u32::<BigEndian>()?;

            // Read the data
            let mut data = vec![0; size as usize];
            file.read_exact(&mut data)?;

            // Generate a tile out of the chunk.
            let chunk = SC2KChunk {
                id: id.clone(),
                length: size,
                data,
            };
            chunks.insert(id.clone(), chunk);

            // Move cursor to next chunk start
            cursor += 4 + 4 + size;
        }

        let city = SC2KCity::default();
        let pict = SC2KPict::default();

        let mut city_data = SC2KCityData {
            file_type,
            length,
            container,
            chunks,
            city,
            pict,
        };

        city_data.extract_city_data();
        city_data.extract_pict_data();

        Ok(city_data)
    }
}

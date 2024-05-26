use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::SC2KChunk;

#[derive(Debug, Default)]
pub struct SC2Map {
    pub tiles: Vec<u16>,
    pub underground_level: u8,
    pub water_level: u8,
    pub land_altitute: u8,
}

impl SC2Map {
    pub fn extract_tiles(chunk: &SC2KChunk) -> io::Result<SC2Map> {
        let mut map = SC2Map::default();
        let mut chunk_data = &chunk.data[0..chunk.data.len()];

        while !chunk_data.is_empty() {
            let tile_bytes = chunk_data.read_u16::<BigEndian>()?;
            map.tiles.push(tile_bytes);
            map.underground_level = SC2Map::extract_underground_level(tile_bytes)?;
            map.water_level = SC2Map::extract_water_level(tile_bytes)?;
            map.land_altitute = SC2Map::extract_land_altitute(tile_bytes)?;
        }

        Ok(map)
    }

    pub fn extract_underground_level(tile: u16) -> io::Result<u8> {
        let underground_level = 0;
        let bit1= SC2Map::extract_bits(tile, 0, 6);
        let bit2= SC2Map::extract_bits(tile, 6, 5);
        let bit3= SC2Map::extract_bits(tile, 11, 5);

        Ok(underground_level)
    }

    pub fn extract_water_level(tile: u16) -> io::Result<u8> {
        let water_level = 0;
        Ok(water_level)
    }

    pub fn extract_land_altitute(tile: u16) -> io::Result<u8> {
        let altitute = 0;
        Ok(altitute)
    }

    pub fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
        let high_byte: u8 = (short_16 >> 8) as u8;
        let low_byte: u8 = (short_16 & 0xff) as u8;
    
        [high_byte, low_byte]
    }

    pub fn extract_bits(value: u16, start: u8, length: u8) -> u16 {
        let mask = (1 << length) - 1;
        (value >> start) & mask
    }
}
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::SC2KChunk;

#[derive(Debug, Default)]
pub struct SC2KMap {
    pub tiles: Vec<u16>,
    pub altitute: u16,
    pub is_water: u16,
}

impl SC2KMap {
    pub fn extract_altm_tiles(chunk: &SC2KChunk) -> io::Result<SC2KMap> {
        let mut map = SC2KMap::default();
        let mut chunk_data = &chunk.data[0..chunk.data.len()];

        while !chunk_data.is_empty() {
            let tile_bytes = chunk_data.read_u16::<BigEndian>()?;
            map.tiles.push(tile_bytes);
            SC2KMap::extract_altm_data(&mut map, tile_bytes)?;
        }

        Ok(map)
    }

    pub fn extract_altm_data(map: &mut SC2KMap, tile: u16) -> io::Result<u8> {
        let underground_level = 0;
        map.altitute = SC2KMap::extract_bits(tile, 0, 4);
        map.is_water = SC2KMap::extract_bits(tile, 7, 1);

        Ok(underground_level)
    }

    pub fn extract_bits(value: u16, start: u8, length: u8) -> u16 {
        let mask = (1 << length) - 1;
        (value >> start) & mask
    }
}

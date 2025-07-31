use std::{collections::HashMap, io};

use byteorder::{BigEndian, ReadBytesExt};

use super::{sc2kfile::SC2KFileChunk, sc2kstats::SC2KStats};

const MAP_SIZE: usize = 128;

/// SimCity 2000 map tile
#[derive(Debug, Default, Clone)]
pub struct SC2KMapTile {
    pub attributes: HashMap<String, u8>,
}

impl SC2KMapTile {
    /// Convert the tile to JSON
    ///
    /// # Returns
    ///
    /// `String` - JSON string
    pub fn to_json(&self) -> String {
        let mut tile = String::from("{");
        for att in &self.attributes {
            tile = format!("{}\"{}\":{},", tile, att.0, att.1);
        }
        tile.pop();
        format!("{}}},", tile)
    }
}

/// SimCity 2000 map data
#[derive(Debug)]
pub struct SC2KMap {
    pub tiles: Vec<Vec<SC2KMapTile>>,
    pub stats: SC2KStats,
    pub name: String,
}

impl Default for SC2KMap {
    fn default() -> Self {
        SC2KMap {
            tiles: vec![vec![SC2KMapTile::default(); MAP_SIZE]; MAP_SIZE],
            stats: SC2KStats::default(),
            name: String::new(),
        }
    }
}

impl SC2KMap {
    /// Extracts the map stats from a MISC chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - MISC chunk from a SimCity 2000 city file.
    ///
    /// # Errors
    ///
    /// * IO error
    pub fn extract_stats(&mut self, chunk: &SC2KFileChunk) -> io::Result<()> {
        self.stats = SC2KStats::new(chunk)?;
        Ok(())
    }

    /// Extracts the map name from a CNAM chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - NAME chunk from a SimCity 2000 city file.
    pub fn extract_name(&mut self, chunk: &SC2KFileChunk) {
        let mut name = String::from("");
        let data = chunk.data.clone();
        for c in data {
            match c {
                31 => continue,
                0 => break,
                _ => name = format!("{}{}", name, c as char),
            }
        }
        name = name.trim().to_string();
        self.name = name.clone();
    }

    /// Extracts the tile altitute data from an ALTM chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - ALTM chunk from a SimCity 2000 file.
    ///
    /// # Errors
    ///
    /// * IO error
    pub fn extract_tiles_altm(&mut self, chunk: &SC2KFileChunk) -> io::Result<()> {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut chunk_data = &chunk.data[0..chunk.data.len()];
        while !chunk_data.is_empty() {
            let tile_bytes = chunk_data.read_u16::<BigEndian>()?;
            self.tiles[y][x].attributes.insert(
                String::from("ALTM"),
                SC2KMap::extract_bits(tile_bytes, 0, 4) as u8,
            );
            self.tiles[y][x].attributes.insert(
                String::from("WATR"),
                SC2KMap::extract_bits(tile_bytes, 7, 1) as u8,
            );
            x += 1;

            if x == MAP_SIZE {
                x = 0;
                y += 1;
            }
        }
        Ok(())
    }

    /// Extracts the tile altitute data from an a one byte chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - one byte chunk from a SimCity 2000 file.
    /// `key` - chunk ID to store the attribute.
    ///
    /// # Errors
    ///
    /// * IO error
    pub fn extract_tiles(&mut self, chunk: &SC2KFileChunk, key: String) -> io::Result<()> {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut chunk_data = &chunk.data[0..chunk.data.len()];
        while !chunk_data.is_empty() {
            self.tiles[y][x]
                .attributes
                .insert(key.clone(), chunk_data.read_u8()?);
            x += 1;

            if x == MAP_SIZE {
                x = 0;
                y += 1;
            }
        }

        Ok(())
    }

    /// Extracts bits from a tile byte to get the altitute or water status.
    ///
    /// # Arguments
    ///
    /// `value` - Tile byte
    /// `start` - Start bit
    /// `length` - Length of bits
    ///
    /// # Returns
    ///
    /// `u16` - Bits from tile byte
    fn extract_bits(value: u16, start: u8, length: u8) -> u16 {
        let mask = (1 << length) - 1;
        (value >> start) & mask
    }

    /// Convert the map to JSON
    ///
    /// # Returns
    ///
    /// `String` - JSON string
    pub fn to_json(&self) -> String {
        let name = format!("\"name\":\"{}\"", self.name);
        let mut tiles = "\"tiles\":[".to_string();

        for y in &self.tiles {
            let mut row = "{\"row\":[".to_string();
            for x in y {
                row = format!("{}{}", row, x.to_json());
            }
            row.pop();
            row = format!("{}]}},", row);
            tiles = format!("{}{}", tiles, row);
        }
        tiles.pop();
        tiles = format!("{}]", tiles);

        let stats = self.stats.to_json();

        format!("{},{},{}", name, stats, tiles)
    }
}

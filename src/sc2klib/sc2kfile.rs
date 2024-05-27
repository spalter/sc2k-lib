use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

use byteorder::{BigEndian, ReadBytesExt};

use super::{sc2kmap::SC2KMap, sc2kpict::SC2KPict};

/// SimCity 2000 chunk
/// Contains either tile data or other information
#[derive(Debug, Default)]
pub struct SC2KFileChunk {
    pub id: String,
    pub length: u32,
    pub data: Vec<u8>,
}

/// SimCity 2000 file reader
///
/// TODO: Missing a few chunk implementations
#[derive(Debug, Default)]
pub struct SC2KFile {
    pub path: String,
    pub chunks: HashMap<String, SC2KFileChunk>,
    pub length: u32,
    pub file_type: u32,
    pub container: u32,
    pub map: SC2KMap,
    pub pict: SC2KPict,
}

impl SC2KFile {
    /// Creates a new SimCity 2000 object from a SimCity 2000 file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the SimCity 2000 file
    ///
    /// # Errors
    ///
    /// * IO error
    pub fn new(path: String) -> io::Result<SC2KFile> {
        let mut sc2k_file = SC2KFile {
            path: path.clone(),
            chunks: HashMap::new(),
            length: 0,
            file_type: 0,
            container: 0,
            map: SC2KMap::default(),
            pict: SC2KPict::default(),
        };

        sc2k_file.read_sc2k_file()?;

        Ok(sc2k_file)
    }

    /// Decrompress the chunk data based on the SimCity 2000 chunk data format.
    /// Which is close to run-length encoding. Everything under 128 is uncompressed.
    /// First byte contains the number of following uncompressed bytes.
    /// In case the first byte is greater than 128, the following byte needs to be
    /// repeated the number of times equal to the first byte minus 127.
    ///
    /// Arguments
    ///
    /// * `c_data` - Chunk data
    ///
    /// Returns
    ///
    /// * `u_data` - Decompressed chunk data
    ///
    /// # Errors
    ///
    /// * IO error
    pub fn decompress_chunk(c_data: Vec<u8>) -> io::Result<Vec<u8>> {
        let mut u_data: Vec<u8> = Vec::new();
        let mut buffer = &c_data[0..c_data.len()];
        let mut counter = 0;
        while counter < c_data.len() {
            let in_number = buffer.read_u8()?;
            counter += 1;
            let in_value = in_number;
            if in_value < 128 {
                for _ in 0..in_value {
                    let value = buffer.read_u8()?;
                    u_data.push(value);
                    counter += 1;
                }
            } else if in_value > 128 {
                let value = buffer.read_u8()?;
                let length = in_value - 127;
                for _n in 0..length {
                    u_data.push(value.clone());
                }
                counter += 1;
            }
        }

        Ok(u_data)
    }

    /// Reads the SimCity 2000 file.
    ///
    /// # Errors
    ///
    /// * IO error
    fn read_sc2k_file(&mut self) -> io::Result<()> {
        let mut file_handle = File::open(self.path.clone())?;
        self.file_type = file_handle.read_u32::<BigEndian>()?;
        self.length = file_handle.read_u32::<BigEndian>()?;
        self.container = file_handle.read_u32::<BigEndian>()?;

        // Extract all chunks
        let mut cursor = 0;
        while cursor < self.length - 4 {
            // Reduce by container close
            // Get the ASCII ID
            let c1 = file_handle.read_u8()?;
            let c2 = file_handle.read_u8()?;
            let c3 = file_handle.read_u8()?;
            let c4 = file_handle.read_u8()?;
            let id = format!("{}{}{}{}", c1 as char, c2 as char, c3 as char, c4 as char);

            // Get the chunk size
            let size = file_handle.read_u32::<BigEndian>()?;

            // Read the data
            let mut c_data = vec![0; size as usize];
            file_handle.read_exact(&mut c_data)?;

            let u_data;
            if id == "CNAM" || id == "ALTM" || id == "PICT" {
                u_data = c_data;
            } else {
                u_data = SC2KFile::decompress_chunk(c_data)?;
            }

            // Generate a tile out of the chunk.
            let chunk = SC2KFileChunk {
                id: id.clone(),
                length: size,
                data: u_data,
            };

            match id.as_str() {
                "PICT" => self.pict.extract_data(&chunk)?,
                "CNAM" => self.map.extract_name(&chunk),
                "MISC" => self.map.extract_stats(&chunk)?,
                "ALTM" => self.map.extract_tiles_altm(&chunk)?,
                "XBLD" => self.map.extract_tiles_xbld(&chunk)?,
                "XBIT" => self.map.extract_tiles_xbit(&chunk)?,
                "XTER" => self.map.extract_tiles_xter(&chunk)?,
                "XUND" => self.map.extract_tiles_xund(&chunk)?,
                "XZON" => self.map.extract_tiles_xzon(&chunk)?,
                _ => {}
            }

            self.chunks.insert(id.clone(), chunk);

            // Move cursor to next chunk start
            cursor += 4 + 4 + size;
        }

        Ok(())
    }

    /// Convert the map data to JSON
    ///
    /// # Returns
    ///
    /// `String` - JSON string
    pub fn to_json(&self) -> String {
        format!("{{{}}}", self.map.to_json())
    }
}

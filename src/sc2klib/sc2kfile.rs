use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{sc2kmap::SC2KMap, sc2kpict::SC2KPict};

/// SimCity 2000 chunk
/// Contains either tile data or other information
#[derive(Debug, Default)]
pub struct SC2KFileChunk {
    pub id: String,
    pub length: u32,
    pub data: Vec<u8>,
}

impl SC2KFileChunk {
    pub fn new(id: String, data: Vec<u8>) -> SC2KFileChunk {
        SC2KFileChunk {
            id,
            length: data.len() as u32,
            data,
        }
    }
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
    pub fn from(path: String) -> io::Result<SC2KFile> {
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
                    u_data.push(value);
                }
                counter += 1;
            }
        }

        Ok(u_data)
    }

    /// Compresses a chunk of data.
    ///
    /// A variant of Run-Length Encoding. Comes in two variants, a 1 + n byte version and a two byte version.
    ///
    /// 1 + n byte version: first byte is in range [0 .. 127] means that there are n bytes of uncompressed data corresponding to the byte’s value after it.
    /// 2 byte version: first byte is in range [129 .. 255] means to repeat the next byte n times (the RLE part), where (byte value) - 127 = n.
    ///
    /// Note that this encoding scheme can lead to certain sections being larger than if they were uncompressed if they’re very random or alternating data.
    /// It appears that every byte is compressed, even if it’s a single byte on its own. There are lots of 0x01 followed by single bytes. Runs are compressed as normal.
    fn compress_chunk(&self, chunk: &SC2KFileChunk) -> Vec<u8> {
        let mut data = chunk.data.clone();
        let mut compressed_data = Vec::new();
        let mut last_byte = data.remove(0);
        let mut counter = 1;
        for byte in data {
            if byte == last_byte {
                if counter == 127 {
                    compressed_data.push(128 + counter);
                    compressed_data.push(last_byte);
                    last_byte = byte;
                    counter = 1;
                } else {
                    counter += 1;
                }
            } else {
                if counter > 127 {
                    compressed_data.push(128 + counter);
                    compressed_data.push(last_byte);
                } else {
                    for _ in 0..counter - 1 {
                        compressed_data.push(last_byte);
                    }
                }

                last_byte = byte;
                counter = 1;
            }
        }

        compressed_data
    }

    /// Saves the SimCity 2000 file.
    pub fn save_sc2k_file(&self, path: &str) -> io::Result<String> {
        let mut file_handle = File::create(path)?;
        file_handle.write_u32::<BigEndian>(self.file_type)?;
        file_handle.write_u32::<BigEndian>(self.length)?;
        file_handle.write_u32::<BigEndian>(self.container)?;

        for chunk in &self.chunks {
            let chunk_name = chunk.1.id.clone();
            for byte in chunk.1.id.as_bytes() {
                file_handle.write_u8(*byte)?;
            }
            file_handle.write_u32::<BigEndian>(chunk.1.length)?;
            if chunk_name != "CNAM" && chunk_name != "ALTM" && chunk_name != "PICT" {
                let compressed_data = self.compress_chunk(chunk.1);
                file_handle.write_all(&compressed_data)?;
            } else if chunk_name == "PICT" {
                let pict_data = self.pict.contract_data();
                file_handle.write_all(&pict_data)?;
            } else if chunk_name == "ALTM" {
                todo!("Implement ALTM contraction")
            } else if chunk_name == "CNAM" {
                todo!("Implement CNAM contraction")
            } else if chunk_name == "MISC" {
                todo!("Implement MISC contraction")
            } else {
                file_handle.write_all(&chunk.1.data)?;
            }
        }

        let file_size = fs::metadata(path).unwrap().len();

        Ok(format!("{}", file_size))
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

            let mut u_data = c_data.clone();
            if id != "CNAM" && id != "ALTM" && id != "PICT" {
                u_data = SC2KFile::decompress_chunk(c_data.clone())?;
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
                "XBLD" => self.map.extract_tiles(&chunk, id.clone())?,
                "XBIT" => self.map.extract_tiles(&chunk, id.clone())?,
                "XTER" => self.map.extract_tiles(&chunk, id.clone())?,
                "XUND" => self.map.extract_tiles(&chunk, id.clone())?,
                "XZON" => self.map.extract_tiles(&chunk, id.clone())?,
                "XPLC" => self.map.extract_tiles(&chunk, id.clone())?,
                "XFIR" => self.map.extract_tiles(&chunk, id.clone())?,
                "XPOP" => self.map.extract_tiles(&chunk, id.clone())?,
                "XROG" => self.map.extract_tiles(&chunk, id.clone())?,
                "XPLT" => self.map.extract_tiles(&chunk, id.clone())?,
                "XVAL" => self.map.extract_tiles(&chunk, id.clone())?,
                "XCRM" => self.map.extract_tiles(&chunk, id.clone())?,
                "XTRF" => self.map.extract_tiles(&chunk, id.clone())?,
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

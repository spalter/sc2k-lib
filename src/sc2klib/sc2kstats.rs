use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use super::sc2kfile::SC2KFileChunk;

/// SimCity 2000 stats
#[derive(Debug, Default)]
pub struct SC2KStats {
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

impl SC2KStats {
    pub fn new(chunk: &SC2KFileChunk) -> io::Result<SC2KStats> {
        let mut stats = SC2KStats::default();
        stats.extract_stats(chunk);
        Ok(stats)
    }

    /// Extracts the stats from a chunk.
    ///
    /// # Arguments
    ///
    /// `chunk` - MISC chunk from a SimCity 2000 city file.
    fn extract_stats(&mut self, chunk: &SC2KFileChunk) {
        let mut chunk_data = &chunk.data[0..chunk.data.len()];
        self.header = chunk_data.read_u32::<BigEndian>().unwrap();
        self.mode = chunk_data.read_u32::<BigEndian>().unwrap();
        self.rotation = chunk_data.read_u32::<BigEndian>().unwrap();
        self.year_founded = chunk_data.read_u32::<BigEndian>().unwrap();
        self.age = chunk_data.read_u32::<BigEndian>().unwrap();
        self.money = chunk_data.read_u32::<BigEndian>().unwrap();
        self.bonds = chunk_data.read_u32::<BigEndian>().unwrap();
        self.level = chunk_data.read_u32::<BigEndian>().unwrap();
        self.status = chunk_data.read_u32::<BigEndian>().unwrap();
        self.city_value = chunk_data.read_u32::<BigEndian>().unwrap();
        self.land_value = chunk_data.read_u32::<BigEndian>().unwrap();
        self.crime_count = chunk_data.read_u32::<BigEndian>().unwrap();
        self.traffic_count = chunk_data.read_u32::<BigEndian>().unwrap();
        self.pollution = chunk_data.read_u32::<BigEndian>().unwrap();
        self.city_fame = chunk_data.read_u32::<BigEndian>().unwrap();
        self.advertising = chunk_data.read_u32::<BigEndian>().unwrap();
        self.garbage = chunk_data.read_u32::<BigEndian>().unwrap();
        self.work_force_percent = chunk_data.read_u32::<BigEndian>().unwrap();
    }

    /// Converts the stats to a JSON string.
    ///
    /// # Returns
    ///
    /// `String` - JSON string
    pub fn to_json(&self) -> String {
        let stats = format!(
                "{{\"mode\":{},\"year_founded\":{},\"age\":{},\"money\":{},\"bonds\":{},\"level\":{},\"status\":{}}}",
                self.mode, self.year_founded, self.age, self.money, self.bonds, self.level, self.status
        );
        format!("\"stats\":{}", stats)
    }
}

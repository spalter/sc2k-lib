//! SimCity 2000 save file library.
//! 
//! Provides an interface to read SimCity 2000 save files.
//! With additional json output and debug information.
//! 
//! # Example
//! ```rust
//! let city_data = SC2KFile::new("./sc2kcity.sc2")?;
//! println!("{:?}", city_data.to_json());
//! ```

/// SimCity 2000 file reader, this is the core library
pub mod sc2kfile;

/// SimCity 2000 map information
pub mod sc2kmap;

/// SimCity 2000 scenario picture
pub mod sc2kpict;

/// SimCity 2000 stats
pub mod sc2kstats;

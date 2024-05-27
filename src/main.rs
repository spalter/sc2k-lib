pub mod sc2kreader;

use std::env;

use crate::sc2kreader::sc2kfile::SC2KFile;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for file in args {
        println!("Load {:?}", file);
        let city_data = SC2KFile::new(file)?;
        println!("{:?}", city_data.map.stats);
    }

    Ok(())
}

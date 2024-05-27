pub mod sc2klib;

use crate::sc2klib::sc2kfile::SC2KFile;

fn main() -> std::io::Result<()> {
    let pattern = std::env::args().nth(1).expect("Missing action e.g. --json or --debug.");

    match pattern.as_str() {
        "-d" | "--debug" => {
            let path = std::env::args().nth(2).expect("Missing file path eg. cities/my_city.sc2.");
            println!("Load {:?}", &path);
            let city_data = SC2KFile::new(path)?;
            println!("{:?}", city_data.map.stats);
        }
        "-j" | "--json" => {
            let path = std::env::args().nth(2).expect("Missing file path eg. cities/my_city.sc2.");
            let city_data = SC2KFile::new(path)?;
            println!("{}", city_data.to_json());
        }
        _ => {}
    }

    Ok(())
}

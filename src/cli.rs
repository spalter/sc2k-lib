extern crate sc2klib;

use std::process::exit;

use sc2klib::sc2kfile::SC2KFile;

fn main() -> std::io::Result<()> {
    if std::env::args().len() < 3 {
        println!(
            "Usage: {} <action> <file>",
            std::env::args().nth(0).unwrap()
        );
        exit(1);
    }

    let pattern = std::env::args()
        .nth(1)
        .expect("Missing action e.g. --json or --debug.");

    let path = std::env::args()
        .nth(2)
        .expect("Missing file path eg. cities/my_city.sc2.");

    match pattern.as_str() {
        "-d" | "--debug" => {
            println!("Try load: {:?}", &path);
            let city_data = SC2KFile::new(path)?;
            println!("{:?}", city_data.map.stats);
        }
        "-j" | "--json" => {
            let city_data = SC2KFile::new(path)?;
            println!("{}", city_data.to_json());
        }
        _ => {}
    }

    Ok(())
}

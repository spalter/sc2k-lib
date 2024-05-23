pub mod sck2data;
pub mod sc2kcity;
pub mod sc2kpict;

use std::env;

use sck2data::*;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for file in args {
        println!("Load {:?}", file);
        let city_data = SC2KCityData::read_sc2k_city_file(file)?;
        println!("{:?}", city_data.city);
    }

    Ok(())
}

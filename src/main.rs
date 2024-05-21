pub mod sck2data;
pub mod sc2kcity;
pub mod sc2kpict;

use sck2data::*;

fn main() -> std::io::Result<()> {
    let file_path = "cities/Utopia.sc2";
    let city_data = SC2KCityData::read_sc2k_city_file(file_path)?;

    println!("{:?}", city_data.city);

    Ok(())
}

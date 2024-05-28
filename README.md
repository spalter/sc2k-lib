# SC2K Library [![Rust](https://github.com/spalter/sc2k-lib/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/spalter/sc2k-lib/actions/workflows/rust.yml)
Simple project to read and convert SimCity 2000 files written in Rust.

## Disclaimer
This is an unofficial project not related to Maxis or SimCity 2000. The source code can be found [here](https://github.com/spalter/sc2k-lib). The information in this file does not originate from Maxis, any Maxis employee, or any signatory of a non-disclosure agreement with Maxis, and neither make any claims as to its accuracy or usability for any purpose.

## Usage

### Debug information
This will output the debug information in the console. The city name and a few stats, but no tile information.
```bash
./sc2kcli -d [.SC2]
```

### JSON output
This will output a JSON representation of the file in the console.
```bash
./sc2kcli -j [.SC2]
```

### Lib usage
The library is split into two parts, a CLI part and a Rust crate. Simply create a new struct with the `SC2KFile::from("path")` function. More details can be found in the rust docs.
```rust
use sc2klib::sc2kfile::SC2KFile;

let city_data = SC2KFile::from(String::from("assets/Utopia.sc2")).unwrap();
println!("{:?}", city_data.to_json());
```

## Credits
All implementation details came from the following two sources. They did a great job documenting the file specs!

- [David Moews](https://djm.cc/simcity-2000-info.txt)
- [dfloer](https://github.com/dfloer/SC2k-docs/tree/master)

# SC2K Library [![Rust](https://github.com/spalter/sc2k-lib/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/spalter/sc2k-lib/actions/workflows/rust.yml)
Simple project to read and convert SimCity 2000 files written in Rust.

## Disclaimer
This is a unofficial project and not related to Maxis or SimCity 2000. The source code can be found [here](https://github.com/spalter/sc2k-lib). The information in this file does not originate from Maxis, any employee of Maxis, or any signatory of a non-disclosure agreement with Maxis, and neither I nor (I presume) Maxis make any claims as to its accuracy or usability for any purpose.

## Usage

### Debug information
This will output the debug information in the console. City name, a few stats, but no tile information.
```bash
./sc2k-lib -d [.SC2]
```

### JSON output
This will output a JSON representation of the file in the console.
```bash
./sc2k-lib -j [.SC2]
```

## Credits
All implementation details came from the following two sources. They did a gread job documenting the file specs!

- [David Moews](https://djm.cc/simcity-2000-info.txt)
- [dfloer](https://github.com/dfloer/SC2k-docs/tree/master)
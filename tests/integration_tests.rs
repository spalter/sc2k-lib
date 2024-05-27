use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs::File, io::Read, process::Command};

#[test]
fn test_debug_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sc2k-lib")?;
    let mut file_handle = File::open("assets/Utopia.json")?;
    let mut buf: String = String::new();
    file_handle.read_to_string(&mut buf)?;

    cmd.arg("-d").arg("assets/Utopia.sc2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SC2KStats { header: 290, mode: 1, rotation: 0, year_founded: 1900, age: 40971, money: 2208137, bonds: 0, level: 1, status: 6, city_value: 132415, land_value: 352952, crime_count: 76140, traffic_count: 88812, pollution: 53227, city_fame: 0, advertising: 0, garbage: 86680170, work_force_percent: 53 }"));

    Ok(())
}

#[test]
fn test_json_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sc2k-lib")?;
    let mut file_handle = File::open("assets/Utopia.json")?;
    let mut buf: String = String::new();
    file_handle.read_to_string(&mut buf)?;

    cmd.arg("-j").arg("assets/Utopia.sc2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(buf));

    Ok(())
}
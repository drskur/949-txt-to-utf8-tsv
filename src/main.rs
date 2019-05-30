#[macro_use] extern crate clap;

use std::fs::{File};
use std::io::{Result, BufReader, BufRead, BufWriter, Write};
use encoding::all::WINDOWS_949;
use encoding::types::Encoding;
use encoding::DecoderTrap;
use clap::App;

fn main() -> Result<()> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let path = matches.value_of("FILE").unwrap();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let output_path = matches.value_of("OUTPUT").unwrap();
    let tsv = File::create(output_path)?;
    let mut writer = BufWriter::new(tsv);

    loop {
        let mut bytes: Vec<u8> = Vec::new();
        let size = reader.read_until(0x0A as u8, &mut bytes)?;
        if size == 0 {
            break;
        }

        let utf8_str = to_utf8(&bytes).replace("|", "\t");
        writer.write(utf8_str.as_bytes())?;
    }


    Ok(())
}

fn to_utf8(bytes: &[u8]) -> String {
    WINDOWS_949.decode(&bytes, DecoderTrap::Strict).unwrap()
}
use std::fs::{File};
use std::io::{Result, BufReader, BufRead, BufWriter, Write};
use encoding::all::WINDOWS_949;
use encoding::types::Encoding;
use encoding::DecoderTrap;

fn main() -> Result<()> {
    let path = "mart_djy_03.txt";
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let tsv = File::create("mart_djy_03.tsv")?;
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
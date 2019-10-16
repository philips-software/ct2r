use serde_json::Result;

use serde::Serialize;

use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

pub fn start(filename: &str) -> Result<()> {
    parse(&filename).unwrap();
    Ok(())
}

#[derive(Serialize)]
pub struct Output {
    pub name: String,
    pub version: String
}

fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut ifile = File::open(&filename)
                         .expect("unable to open file");
    ifile.read_to_string(&mut input).expect("unable to read");
    input
}

fn write_file(filename: &str, data: Vec<Output>) {
    let output_string = format!("{}", serde_json::to_string_pretty(&data).unwrap());
    let ofile = File::create(&filename)
                         .expect("unable to create file");
    let mut ofile = BufWriter::new(ofile);
    ofile.write_all(output_string.as_bytes()).expect("unable to write");
}

fn parse(filename: &str) -> Result<()> {
    let file_content = read_file(&filename);
    let outputs: Vec<Output> = crate::vendors::xray::parse_file(&file_content);
    let output_filename = "output.json";
     write_file(&output_filename, outputs);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert!(parse("./tests/xray-license-export.json").is_ok());
    }
}

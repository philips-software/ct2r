use serde_json::Result;

use serde::Serialize;

use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

pub fn start(tool: &str, filename: &str, output: &str) -> Result<()> {
    parse(&tool, &filename, &output).unwrap();
    Ok(())
}

#[derive(Serialize)]
pub struct Output {
    pub name: String,
    pub version: String,
}

fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut ifile = File::open(&filename).expect("unable to open file");
    ifile.read_to_string(&mut input).expect("unable to read");
    input
}

fn write_file(filename: &str, data: Vec<Output>) {
    let output_string = serde_json::to_string_pretty(&data).unwrap();
    let ofile = File::create(&filename).expect("unable to create file");
    let mut ofile = BufWriter::new(ofile);
    ofile
        .write_all(output_string.as_bytes())
        .expect("unable to write");
}

fn parse(tool: &str, filename: &str, output: &str) -> Result<()> {
    let file_content = read_file(&filename);
    let outputs: Vec<Output> = match tool {
        "xray" => crate::vendors::xray::parse_file(&file_content),
        "gradle" => crate::vendors::gradle::parse_file(&file_content),
        _ => Vec::new(),
    };

    write_file(&output, outputs);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xray_test() {
        assert!(parse("xray", "./tests/xray-license-export.json", "./output.json").is_ok());
    }

    #[test]
    fn parse_gradle_test() {
        assert!(parse("gradle", "./tests/gradle-license-export.json", "./output.json").is_ok());
    }
}

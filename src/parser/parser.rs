use serde_json::Result;

pub fn example(filename: &str) -> Result<()> {
    xray::parse(&filename).unwrap();
    Ok(())
}

mod xray {
    use serde_json::Result;
    use serde::{ Deserialize, Serialize };

    use std::fs::File;
    use std::io::Read;
    use std::io::{BufWriter, Write};

    #[derive(Serialize)]
    struct Output {
        name: String,
        version: String
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
    
    pub fn parse(filename: &str) -> Result<()> {
        let file_content = read_file(&filename);
        let outputs: Vec<Output> = parse_file(&file_content);

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

    // Xray specific
    
    #[derive(Deserialize, Serialize)]
    struct Xray {
        component_id: String,
        component_name: String,
        version: String,
        pkg_type: String,
        package_id: String,
    }

    fn parse_file(content: &str) -> Vec<(Output)> {
        let components: Vec<Xray> = serde_json::from_str(&content).unwrap();
        let mut outputs: Vec<Output> = Vec::new();
        for component in components.iter() {
            println!("component_id: {}\ncomponent_name: {}\nversion: {}", component.component_id, component.component_name, component.version);
            let output = Output{
                name: component.component_name.to_string(), 
                version: component.version.to_string()
            };
            outputs.push(output);
            println!("-==================================================-");
        }
        outputs
    }

}

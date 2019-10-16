use serde::{ Deserialize, Serialize };

use crate::engine::program::Output;

#[derive(Deserialize, Serialize)]
struct Xray {
    component_id: String,
    component_name: String,
    version: String,
    pkg_type: String,
    package_id: String,
}

pub fn parse_file(content: &str) -> Vec<(Output)> {
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

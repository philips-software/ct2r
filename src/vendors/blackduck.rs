use serde::Deserialize;

use crate::engine::program::Output;

#[derive(Deserialize, Debug)]
struct Blackduck {
    component_name: String,
    component_version_name: String,
    channel_versions: String,
}

pub fn parse_file(content: &str) -> Vec<(Output)> {
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut outputs: Vec<Output> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let output = Output{
            name: record[3].to_string(), 
            version: record[5].to_string()
        };
        outputs.push(output);
    }
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test() {
        let test_input = r#"
Used by,Component id,Version id,Component name,Component version name,Channel versions,License ids,License names,License families,Match type,Producer match type,Usage,In use,Operational Risk,Component policy status,Overridden By,Created At,Origin name,Origin id,Review Status,Reviewed by,Reviewed at,Project path,Source/Type,Snippet Review status
,003caf2f-6255-4165-9f98-c8db8177bade,bff7feeb-40aa-4706-8b24-c4a48659147f,whatwg-encoding,1.0.5,1.0.5,ad705c59-6893-4980-bdbf-0837f1823cc4,MIT License,PERMISSIVE,Transitive Dependency,,DYNAMICALLY_LINKED,true,OK,Not In Violation,,22/10/19,npmjs,whatwg-encoding/1.0.5,NOT_REVIEWED,,,Technology_Whitelisting js-react-app-version_1.0.0,KB_COMPONENT,
,0098563f-673a-4027-91d9-94fbc26e7ae3,35e48b66-ecd6-4335-b0a4-ad9cd93481d3,postcss-selector-not,4.0.0,4.0.0,ad705c59-6893-4980-bdbf-0837f1823cc4,MIT License,PERMISSIVE,Transitive Dependency,,DYNAMICALLY_LINKED,true,HIGH,Not In Violation,,22/10/19,npmjs,postcss-selector-not/4.0.0,NOT_REVIEWED,,,Technology_Whitelisting js-react-app-version_1.0.0,KB_COMPONENT,
"#;
        let result: Vec<Output> = parse_file(test_input);
        assert_eq!(result.len(),2);
    }
}

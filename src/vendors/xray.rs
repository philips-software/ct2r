use serde::{Deserialize, Serialize};

use crate::engine::program::Output;

#[derive(Deserialize, Serialize, Debug)]
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
        let output = Output {
            name: component.component_name.to_string(),
            version: component.version.to_string(),
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
[
   {
      "component_id": "karma-sourcemap-loader:0.3.7",
      "component_name": "karma-sourcemap-loader",
      "version": "0.3.7",
      "pkg_type": "npm",
      "package_id": "npm://karma-sourcemap-loader",
      "licenses": [
         {
            "key": "MIT",
            "link": "http://www.opensource.org/licenses/MIT",
            "sources": [
               {
                  "source": "JFrog",
                  "occurrences": 1
               },
               {
                  "source": "Local File",
                  "occurrences": 2
               }
            ]
         }
      ]
   },
   {
      "component_id": "css-selector-tokenizer:0.7.0",
      "component_name": "css-selector-tokenizer",
      "version": "0.7.0",
      "pkg_type": "npm",
      "package_id": "npm://css-selector-tokenizer",
      "licenses": [
         {
            "key": "MIT",
            "link": "http://www.opensource.org/licenses/MIT",
            "sources": [
               {
                  "source": "JFrog",
                  "occurrences": 1
               },
               {
                  "source": "Local File",
                  "occurrences": 1
               }
            ]
         }
      ]
   }
]
    "#;
        let result: Vec<Output> = parse_file(test_input);
        assert_eq!(result.len(), 2);
    }
}

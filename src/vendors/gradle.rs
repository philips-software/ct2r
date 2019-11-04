use serde::{Deserialize, Serialize};

use crate::engine::program::Output;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GradleDependency {
    dependencies: Vec<GradleModule>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GradleModule {
    module_name: String,
    module_version: String,
}

pub fn parse_file(content: &str) -> Vec<(Output)> {
    let root: GradleDependency = serde_json::from_str(&content).unwrap();
    let mut outputs: Vec<Output> = Vec::new();
    for component in root.dependencies.iter() {
        let output = Output {
            name: component.module_name.to_string(),
            version: component.module_version.to_string(),
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
{
   "dependencies": [
        {
            "moduleName": "org.unbescape:unbescape",
            "moduleUrl": "http://www.unbescape.org",
            "moduleVersion": "1.1.6.RELEASE",
            "moduleLicense": "The Apache Software License, Version 2.0",
            "moduleLicenseUrl": "http://www.apache.org/licenses/LICENSE-2.0.txt"
        },
        {
            "moduleName": "org.yaml:snakeyaml",
            "moduleUrl": "http://www.snakeyaml.org",
            "moduleVersion": "1.23",
            "moduleLicense": "Apache License, Version 2.0",
            "moduleLicenseUrl": "http://www.apache.org/licenses/LICENSE-2.0.txt"
        }
    ]
}
    "#;
        let result: Vec<Output> = parse_file(test_input);
        assert_eq!(result.len(), 2);
    }
}

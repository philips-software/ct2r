[![Build Status](https://github.com/philips-software/ct2r/workflows/build/badge.svg)](https://github.com/philips-software/ct2r/actions/)
[![Slack](https://philips-software-slackin.now.sh/badge.svg)](https://philips-software-slackin.now.sh)

# ct2r - Convert Tool Output to Reference Output 

> Note: :warning: If you're interested in Software Bill of Materials, you might also look into [Bompare](https://github.com/philips-labs/bompare)

## Purpose

We're using several tools to get more insight into the Bill Of Material of applications. All tools and languages have different ways to report these lists with dependencies, licenses and vulnerabilities. In order to compare tools with each other, we need a way to convert the output to a reference output. See reference output section about what is our reference output.
This repository converts outputs from various tools.

### Supported tools

- [x] 1 Manual generated JSON from package managers (f.e. gradle)
- [ ] 2 Blackduck 
- [x] 3 [Jfrog Xray][xray]
- [ ] 4 Whitesource
- [ ] 5 FOSSA
- [ ] 6 Snyk

## Prerequisite 

Install Rust and Cargo

``` bash
curl https://sh.rustup.rs -sSf | sh
```

## Usage

### Release version

Build a release version of the scripts.

```
cargo build --release
```

### Run a script with a test file. 

An example with the [JFrog Xray][xray] license export results json file.

```
./target/release/ct2r xray Build_js-react-app-443222_License_Export.json output.json
```

An example with the gradle license export results json file.
```
./target/release/ct2r gradle raw-dependencies.json output.json
```

#### Generate json from packagemanager gradle

```
buildscript {
    repositories {
        maven {
            url 'https://plugins.gradle.org/m2/'
        }
    }
    dependencies {
        classpath 'com.github.jk1:gradle-license-report:1.11'
    }
}

apply plugin: 'com.github.jk1.dependency-license-report'

import com.github.jk1.license.render.*

licenseReport {
    renderers = [new JsonReportRenderer()]
}
```

Run:

```
./gradlew generateLicenseReport
```

### Debug version

```
cargo run <tool> <input.json> <output.json>
```

Tool can be: `xray` or `gradle`.


### Run tests
```
cargo test
```

### Docker version

Run tool from docker image:
```
docker  run --rm -v (pwd):/data philipssoftware/ct2r gradle /data/index.json /data/output.json
```

## Reference Output

The output is saved in a file: `output.json`
It's format is aligned with the reference output found in tools like: [npm dependencies extractor][nde]

```
[
   { ‘name’: ‘dependencyName_A’, ‘version’: ‘0.0.1’},
   { ‘name’: ‘dependencyName_B’, ‘version’: ‘2.0.0’},
   { ‘name’: ‘dependencyName_B’, ‘version’: ‘2.0.1’}
]
```

## Author

Jeroen Knoops

## Future Features 
- Complement the output of one tool with the other.

[nde]: https://github.com/philips-software/npm-dependencies-extractor
[xray]: https://jfrog.com/xray/

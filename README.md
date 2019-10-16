# ct2r - Convert Tool Output to Reference Output 

## Purpose

This repository holds a tool which can be used to convert outputs of various tools to the reference output.

### Supported tools

- [ ] 1 manual 
- [ ] 2 blackduck 
- [x] 3 [Jfrog Xray][xray]
- [ ] 4 whitesource

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
./target/release/ct2r Build_js-react-app-443222_License_Export.json
```

### Debug version

```
cargo run <filename.json>
```

### Run tests
```
cargo test
```

## Output

The output is saved in a file: `output.json`
It's format is aligned with the output found in tools like: [npm dependencies extractor][nde]

```
[
   { ‘name’: ‘dependencyName_A’, ‘version’: ‘0.0.1’},
   { ‘name’: ‘dependencyName_B’, ‘version’: ‘2.0.0’},
   { ‘name’: ‘dependencyName_B’, ‘version’: ‘2.0.1’}
]
```

## Author

Jeroen Knoops

[nde]: https://github.com/philips-software/npm-dependencies-extractor
[xray]: https://jfrog.com/xray/

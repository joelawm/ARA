# ARA (Automatic Rust API)
This program is extremely simple in nature. It is designed to run through your program and create a graph of all the function calls and returns. This in return will give the program a understanding of the hierarchy and will allow us to generate a API Documentation based off what is coded and not what comment is written. The problem I had with a lot of existing tools is that it requires the user to write a comment to generate the documentation. This is not ideal as the comment can be outdated or incorrect. This program will generate the documentation based off the code itself instead.

## Features:
- [x] Parse Rust code
- [x] Parse Rust Workspace
- [x] Output API documentation to terminal
- [ ] Output OpenAPI documentation
- [ ] Create Typescript interfaces based on the API's documentations

### Configuration Parameters:
- `ignore` - Give a list of folders/file paths to ignore such as `["/target", "/build"]`
- `function_name` - Only run the test for function/method definition with the given name. This is useful for debugging a specific function/method.
- `debug` - Print debug information to the terminal.
- `verbose` - Print verbose debug information to the terminal.
- `path` - Path to the root of the project. This is useful when running the program outside of the project root.

### Run Locally:
1. Run the program using cargo
```bash
cargo run
```
***Note: Parameters are optional to run the program. If not provided, default values will be used.***

### Run Tests:
1. Run the tests using cargo
```bash
cargo test
```
Note: A Common way for me to debug is doing the following, this way I get all the relevant information in the terminal.
```bash
clear && cargo run -- --path ../backend --debug -v
```

### Production Deployment:
1. Build the program using cargo
```bash
cargo build --release
```
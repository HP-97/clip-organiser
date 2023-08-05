# rs-cli-template
A subjective template to starting a CLI program written in Rust.

## Directory layout

This project makes use of **Cargo Workspaces** to organise the codebase.

### bin

The 'bin' package contains `main.rs` and is where the `main` function lives.

### cli

The 'cli' package contains the definition of the CLI interface, including docs for every flag.

### core

The 'core' package contains **all** of the business logic of the program. The package name is 'business_core' 

### utils

The 'utils' package contains miscellaneous logic i.e. logging and configuration.
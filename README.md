# Agave Checkout Generator

This tool helps generate sparse checkout commands for Agave packages, making it easier to work with specific packages from the Agave repository.

## Prerequisites

- Rust and Cargo installed
- Git
- Agave repository cloned in the parent directory (`../agave`) or in a custom location

## Setup

1. Clone the Agave repository in the parent directory of this project (or your preferred location):
```bash
cd ..
git clone https://github.com/anza-xyz/agave.git
```

The default directory structure should look like this:
```
parent-directory/
├── agave/                    # Full Agave repository
└── agave-checkout-gen/      # This tool
```

If you want to use a different location for the Agave repository, you can modify the `AGAVE_PATH` constant in `src/bin/extract_packages.rs`:
```rust
const AGAVE_PATH: &str = "./../agave";  // Change this to your preferred path
```

## Usage

### 1. Generate Package Information
First, generate a JSON file containing all packages and their dependencies:
```bash
cargo run --bin extract_packages
```
This command will analyze the Agave repository (using the path specified in `extract_packages.rs`) to generate the package information.

### 2. Generate Git Checkout Command
Generate the git sparse checkout command:
```bash
cargo run --bin create_git_command
```

### 3. Clone the Repository
Clone the Agave repository with minimal blob data:
```bash
git clone --filter=blob:none --sparse https://github.com/anza-xyz/agave.git <PROJECT_NAME>
```

### 4. Apply Sparse Checkout
Navigate to the cloned repository and apply the generated checkout command:
```bash
cd agave-solana-svm
# Copy and run the command from sparse_checkout_command.sh
```

### 5. Build Specific Packages
Build individual packages using cargo:
```bash
cargo build --lib --package <PACKAGE_NAME>
# Example: cargo build --lib --package solana-svm
```

## Current Status

This project is currently a work in progress. Next steps:

- Evaluate if all necessary dependencies are included
- Generate a updated Cargo.toml file or a script to update the Cargo.toml file or changes to add to the Cargo.toml file
- Check if we can run the tests for the packages


# Agave Checkout Generator

This tool helps generate sparse checkout commands for Agave packages, making it easier to work with specific packages from the Agave repository.

## Prerequisites

- Rust and Cargo installed
- Git
- Agave repository (will be needed for to run the scripts)

## Setup

### 1. Clone this Repository
```bash
git clone https://github.com/janinedotgm/agave-checkout-gen.git
cd agave-checkout-gen
```

### 2. Configure Agave Repository Path
Update the path to your Agave repository in `src/bin/extract_packages.rs`:
```rust
const AGAVE_PATH: &str = "<PATH_TO_YOUR_AGAVE_REPO>";  // Update this path
```

Don't have the Agave repository yet? You can clone it from:
```bash
git clone https://github.com/anza-xyz/agave.git
```

The default setup expects the following structure, but you can use any location by updating the path above:
```
parent-directory/
├── agave/                    # Agave repository
└── agave-checkout-gen/      # This tool
```

## Usage

### 1. Generate Package Information
Generate a JSON file containing all packages and their dependencies:
```bash
cargo run --bin extract_packages
```

### 2. Generate Git Checkout Command
Generate the git sparse checkout command:
```bash
cargo run --bin create_git_command <PACKAGE_NAME>
# example: cargo run --bin create_git_command solana-svm
```

### 3. Clone the Repository
Clone the Agave repository with minimal blob data:
```bash
git clone --filter=blob:none --sparse https://github.com/anza-xyz/agave.git <PROJECT_NAME>
```

### 4. Apply Sparse Checkout
Navigate to the cloned repository and apply the generated checkout command:
```bash
cd <PROJECT_NAME>
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


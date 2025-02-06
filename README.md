# Agave Package Isolator

A tool that helps you isolate and work with specific packages from the Agave repository by generating optimized sparse checkout commands and compatible Cargo.toml configurations.

## Prerequisites

- Rust and Cargo installed
- Git
- Agave repository (will be needed for analysis)

## Setup

### 1. Clone this Repository
```bash
git clone https://github.com/janinedotgm/agave-checkout-gen.git
cd agave-checkout-gen
```

### 2. Configure Agave Repository Path
Update the path to your Agave repository in `src/constants.rs`:
```rust
pub const AGAVE_PATH: &str = "<PATH_TO_YOUR_AGAVE_REPO>";  // Update this path
```

Don't have the Agave repository yet? You can clone it from:
```bash
git clone https://github.com/anza-xyz/agave.git
```

## Usage

### 1. Build Project
```bash
cargo build
```

### 2. Generate Files
Run the program with your target package:
```bash
cargo run <PACKAGE_NAME>
# Example: cargo run solana-svm
```

This will:
- Analyze the package and its dependencies
- Generate a sparse checkout command
- Create an optimized Cargo.toml for your isolated package

The output files will be in the `output` directory:
- `sparse_checkout_command.sh`: Contains the git sparse-checkout command
- `Cargo.toml`: The optimized Cargo.toml file for your isolated package

### 2. Set Up Your Isolated Package

1. Clone the Agave repository with minimal blob data:
```bash
git clone --filter=blob:none --sparse https://github.com/anza-xyz/agave.git <PROJECT_NAME>
```

2. Apply Sparse Checkout
Copy and run the command from output/sparse_checkout_command.sh 
```bash
cd <PROJECT_NAME>
```

3. Replace Cargo.toml
Replace the `Cargo.toml` in the root directory of your isolated agave repo with the one generated in `output/Cargo.toml`

4. Build Your Package
```bash
cargo build --lib --package <PACKAGE_NAME>
```

## Current Status

This project is currently a work in progress. Next steps:

- Evaluate if all necessary dependencies are included
- Check if we can run the tests for the packages
- Test other packages (so far only tested with `solana-svm`)
- The generated `Cargo.toml` formatting could be improved

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
Update the path to your Agave repository in `src/constants.rs`:
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
cargo run --bin extract-packages
```

### 2. Generate Git Checkout Command
Generate the git sparse checkout command:
```bash
cargo run --bin create-git-command <PACKAGE_NAME>
# example: cargo run --bin create_git_command solana-svm
```

### 3. Generate new workspace members list and patches dependencies
Since you only check out the folders for this specific package, you have to update the workspace 
members list and patches dependencies in your `Cargo.toml`. You can use the `update-cargo-toml` script to generate a members 
list that only contains the folders you checked out and a patches.toml file that only contains the patches dependencies for the checked out packages.

The following command creates a new members list in `output/members.toml` and a patches.toml file in `output/patches.toml`.
```bash
cargo run --bin update-cargo-toml
```

### 4. Clone the Repository
Clone the Agave repository with minimal blob data:
```bash
git clone --filter=blob:none --sparse https://github.com/anza-xyz/agave.git <PROJECT_NAME>
```

### 6. Apply Sparse Checkout
Navigate to the cloned repository and apply the generated checkout command:
```bash
cd <PROJECT_NAME>
# Copy and run the command from sparse_checkout_command.sh
```

### 7. Replace Members Array
Replace the workspace members list in the `Cargo.toml` with the one generated in step 3 (`output/members.toml`).
```rust
// replace this part with the content of output/members.toml
members = [
    // ...
]
```

### 8. Replace Patches
Replace the patches in the `Cargo.toml` with the one generated in step 3 (`output/patches.toml`).
```rust
// replace this part with the content of output/patches.toml
[patch.crates-io]
// ...
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
- Check if we can run the tests for the packages
- For some packages I had to also remove lines from `[patch.crates-io]`. Example: For `solana-svm` I had to remove `solana-zk-sdk = { path = "zk-sdk" }` to compile it successfully. 


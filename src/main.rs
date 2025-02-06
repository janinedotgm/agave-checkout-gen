use std::{env, fs};

mod processors;
mod constants;

use processors::{
    package_analyzer,
    git_command,
    cargo_toml
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get package name from command line arguments
    let args: Vec<String> = env::args().collect();
    let package_name = args.get(1).expect("Please provide a package name");

    // Create output directory if it doesn't exist
    fs::create_dir_all("output")?;

    println!("Step 1: Extracting package information...");
    let packages = package_analyzer::extract_packages()?;

    println!("\nStep 2: Creating git sparse-checkout command...");
    git_command::create_git_command(package_name, &packages)?;

    println!("\nStep 3: Updating Cargo.toml...");
    let checkout_command = fs::read_to_string("./output/sparse_checkout_command.sh")?;
    cargo_toml::update_cargo_toml(&checkout_command)?;

    println!("\nAll steps completed successfully!");
    println!("1. sparse-checkout command is in output/sparse_checkout_command.sh");
    println!("2. Updated Cargo.toml is in output/Cargo.toml");
    
    Ok(())
} 
use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;
use agave_checkout_gen::processors::{
    package_analyzer,
    git_command,
    cargo_toml
};

/// Tests actual compilation of a package specified via PACKAGE env var.
/// Example: PACKAGE=solana-svm cargo test compilation_test -- --ignored
#[test]
fn test_package_compilation() -> Result<(), Box<dyn std::error::Error>> {
    let package_name = env::var("PACKAGE").unwrap_or_else(|_| {
        println!("No PACKAGE environment variable set, defaulting to solana-svm");
        "solana-svm".to_string()
    });

    println!("Testing compilation for package: {}", package_name);
    let test_dir = Path::new("test_checkout");

    // Cleanup any previous test
    if test_dir.exists() {
        fs::remove_dir_all(test_dir)?;
    }

    // Create fresh test directory
    fs::create_dir_all(test_dir)?;

    println!("Cloning repository...");
    // Clone Agave repo
    let clone_status = Command::new("git")
        .args([
            "clone",
            "--filter=blob:none",
            "--sparse",
            "https://github.com/anza-xyz/agave.git",
            test_dir.to_str().unwrap()
        ])
        .status()?;
    assert!(clone_status.success(), "Failed to clone repository");

    // Generate necessary files using our tool
    let packages = package_analyzer::extract_packages()?;

    // Verify package exists
    if !packages.contains_key(&package_name) {
        return Err(format!("Package '{}' not found in workspace", package_name).into());
    }

    // Generate sparse-checkout command
    git_command::create_git_command(&package_name, &packages)?;
    let checkout_command = fs::read_to_string("./output/sparse_checkout_command.sh")?;

    println!("Running sparse checkout...");
    // Run sparse-checkout directly in the test directory
    let checkout_status = Command::new("sh")
        .current_dir(test_dir)
        .arg("-c")
        .arg(&checkout_command)
        .status()?;
    assert!(checkout_status.success(), "Sparse checkout failed");

    // Now generate and copy the Cargo.toml
    println!("Updating Cargo.toml...");
    cargo_toml::update_cargo_toml(&checkout_command)?;
    fs::copy(
        "output/Cargo.toml",
        test_dir.join("Cargo.toml")
    )?;

    println!("Building package...");
    // Try to build the package
    let build_status = Command::new("cargo")
        .current_dir(test_dir)
        .args(["build", "--lib", "--package", &package_name])
        .status()?;
    assert!(build_status.success(), "Package failed to compile");

    println!("Cleaning up...");
    // Cleanup
    fs::remove_dir_all(test_dir)?;

    println!("Successfully tested package: {}", package_name);
    Ok(())
}
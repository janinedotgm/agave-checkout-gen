use std::{fs, path::Path};
use serde::{Serialize, Deserialize};
use agave_checkout_gen::processors::package_analyzer;
use agave_checkout_gen::constants::AGAVE_PATH;
use agave_checkout_gen::processors::{
  git_command,
  cargo_toml
};

#[derive(Serialize, Deserialize)]
struct TestResults {
    successful: Vec<String>,
    failed: Vec<(String, String)>, // (package_name, error_message)
    remaining: Vec<String>,
}

fn get_package_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let package_list_path = Path::new(manifest_dir).join("tests/package_list.txt");
    let content = fs::read_to_string(package_list_path)?;
    Ok(content.lines().map(String::from).collect())
}

#[test]
fn test_all_packages() -> Result<(), Box<dyn std::error::Error>> {
    let test_output = Path::new("tests/output");
    fs::create_dir_all(test_output)?;
    
    let results_path = test_output.join("test_results.json");
    let results_path_ref = results_path.as_path();  // Create reference to path
    let mut results = if results_path_ref.exists() {
        println!("Found existing results, resuming...");
        let content = fs::read_to_string(results_path_ref)?;
        serde_json::from_str(&content)?
    } else {
        let packages = get_package_list()?;
        TestResults {
            successful: Vec::new(),
            failed: Vec::new(),
            remaining: packages,
        }
    };

    println!("Packages to test: {}", results.remaining.len());
    println!("Already successful: {}", results.successful.len());
    println!("Previously failed: {}", results.failed.len());

    while let Some(package) = results.remaining.pop() {
        println!("\nTesting package: {}", package);
        
        match test_package(&package) {
            Ok(()) => {
                println!("✅ Success: {}", package);
                results.successful.push(package);
            }
            Err(e) => {
                println!("❌ Failed: {} - {}", package, e);
                results.failed.push((package, e.to_string()));
            }
        }

        // Save progress after each package
        fs::write(
            results_path_ref,
            serde_json::to_string_pretty(&results)?
        )?;
    }

    // Generate final report
    generate_report(&results)?;
    Ok(())
}

fn test_package(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    
    println!("Testing compilation for package: {}", package_name);
    
    // Create test output directory structure
    let test_output = Path::new("tests/output");
    let test_dir = test_output.join("test_checkout");
    let test_dir_path = test_dir.as_path();  // Create reference to path
    
    // Cleanup any previous test
    if test_dir_path.exists() {
        fs::remove_dir_all(test_dir_path)?;
    }
    fs::create_dir_all(test_dir_path)?;

  println!("Cloning repository...");
  // Clone Agave repo
  let clone_status = Command::new("git")
      .args([
          "clone",
          "--filter=blob:none",
          "--sparse",
          "https://github.com/anza-xyz/agave.git",
          test_dir_path.to_str().unwrap()
      ])
      .status()?;
  assert!(clone_status.success(), "Failed to clone repository");

  // Generate necessary files using our tool
  let packages = package_analyzer::extract_packages()?;

  // Verify package exists
  if !packages.contains_key(package_name) {
      return Err(format!("Package '{}' not found in workspace", package_name).into());
  }

  // Generate sparse-checkout command
  git_command::create_git_command(&package_name, &packages)?;
  let checkout_command = fs::read_to_string("./output/sparse_checkout_command.sh")?;

  println!("Running sparse checkout...");
  // Run sparse-checkout directly in the test directory
  let checkout_status = Command::new("sh")
      .current_dir(test_dir_path)
      .arg("-c")
      .arg(&checkout_command)
      .status()?;
  assert!(checkout_status.success(), "Sparse checkout failed");

  // Now generate and copy the Cargo.toml
  println!("Updating Cargo.toml...");
  cargo_toml::update_cargo_toml(&checkout_command)?;
  fs::copy(
      "output/Cargo.toml",
      test_dir_path.join("Cargo.toml")
  )?;

  println!("Building package...");
  // Try to build the package
  let build_status = Command::new("cargo")
      .current_dir(test_dir_path)
      .args(["build", "--lib", "--package", &package_name])
      .status()?;
  assert!(build_status.success(), "Package failed to compile");

  println!("Cleaning up...");
  // Cleanup
  fs::remove_dir_all(test_dir_path)?;

  println!("Successfully tested package: {}", package_name);
  Ok(())
}

fn generate_report(results: &TestResults) -> Result<(), Box<dyn std::error::Error>> {
    let test_output = Path::new("tests/output");
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut report = String::new();
    report.push_str(&format!("# Package Test Results ({})\n\n", timestamp));
    
    report.push_str("## Summary\n");
    report.push_str(&format!("- Total packages tested: {}\n", 
        results.successful.len() + results.failed.len()));
    report.push_str(&format!("- Successful: {}\n", results.successful.len()));
    report.push_str(&format!("- Failed: {}\n", results.failed.len()));
    report.push_str(&format!("- Remaining: {}\n\n", results.remaining.len()));
    
    if !results.successful.is_empty() {
        report.push_str("## Successful Packages\n");
        for package in &results.successful {
            report.push_str(&format!("- ✅ {}\n", package));
        }
        report.push_str("\n");
    }
    
    if !results.failed.is_empty() {
        report.push_str("## Failed Packages\n");
        for (package, error) in &results.failed {
            report.push_str(&format!("- ❌ {}: {}\n", package, error));
        }
    }
    
    fs::write(test_output.join("test_report.md"), report)?;
    Ok(())
} 
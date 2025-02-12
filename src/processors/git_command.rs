use std::{collections::HashSet, fs::File, io::Write};
use crate::processors::package_analyzer::PackageInfo;

pub fn create_git_command(package_name: &str, packages: &std::collections::HashMap<String, PackageInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let mut collected_deps = HashSet::new();
    collect_dependencies(package_name, packages, &mut collected_deps);
    
    // Add the package's own path
    if let Some(module_data) = packages.get(package_name) {
        collected_deps.insert(module_data.path.clone());
    }

    // Create output file
    let mut output_file = File::create("./output/sparse_checkout_command.sh")?;

    // Write the git command
    writeln!(output_file, "git sparse-checkout set \\")?;

    // Write paths to the output file
    for path in collected_deps {
        writeln!(output_file, "    {} \\", path)?;
    }

    Ok(())
}

fn collect_dependencies(
    module: &str,
    dependencies: &std::collections::HashMap<String, PackageInfo>,
    collected_deps: &mut HashSet<String>
) {
    // Skip if module name contains dots (relative paths)
    if module.contains(".") {
        println!("Skipping module with name '.' or '..'");
        return;
    }

    // Add the module name without the "solana-" prefix to collected dependencies
    collected_deps.insert(module.replace("solana-", "").to_string());

    if let Some(module_data) = dependencies.get(module) {
        // Add regular dependencies
        for (key, value) in &module_data.dependencies {
            if collected_deps.insert(value.clone()) {
                collect_dependencies(key, dependencies, collected_deps);
            }
        }

        // Add development dependencies
        for (key, value) in &module_data.dev_dependencies {
            if !value.contains(".") && collected_deps.insert(value.clone()) {
                collect_dependencies(key, dependencies, collected_deps);
            }
        }
    }
} 
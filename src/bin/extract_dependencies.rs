use std::collections::HashSet;
use std::fs;
use std::io::Write;
use serde_json::Value;

fn get_all_dependencies(package: &str, deps_map: &serde_json::Map<String, Value>, collected: &mut HashSet<String>) {
    // Add the package itself to the collected set
    if !collected.contains(package) {
        collected.insert(package.to_string());
    }

    if let Some(package_info) = deps_map.get(package) {
        // Process regular dependencies
        if let Some(dependencies) = package_info.get("dependencies")
            .and_then(|deps| deps.as_object()) {
            for (dep_name, _) in dependencies {
                if !collected.contains(dep_name) {
                    collected.insert(dep_name.clone());
                    if dep_name.contains("svm") {
                        println!("naaaaaame: {:?}", dep_name);
                    }
                    // Recursively get dependencies of this dependency
                    get_all_dependencies(dep_name, deps_map, collected);
                }
            }
        }

        // Process dev-dependencies
        if let Some(dev_dependencies) = package_info.get("dev_dependencies")
            .and_then(|deps| deps.as_object()) {
            for (dep_name, _) in dev_dependencies {
                if !collected.contains(dep_name) {
                    collected.insert(dep_name.clone());
                    if dep_name.contains("svm") {
                        println!("naaaaaame: {:?}", dep_name);
                    }
                    // Recursively get dependencies of this dev-dependency
                    get_all_dependencies(dep_name, deps_map, collected);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the JSON file
    let json_content = fs::read_to_string("./output/packages_with_path.json")?;
    let mut deps_map: serde_json::Map<String, Value> = serde_json::from_str(&json_content)?;
    for value in deps_map.keys() {
        println!("{}", value);
    }

    let mut all_required_packages = HashSet::new();
    get_all_dependencies("solana-svm", &deps_map, &mut all_required_packages);
    // Create output file
    let mut output_file = fs::File::create("./output/sparse_checkout_command.sh")?;

    // Write the git command
    writeln!(output_file, "git sparse-checkout set \\")?;

    for package in all_required_packages {
        if let Some(package_info) = deps_map.get(&package) {
            if let Some(path) = package_info.get("path").and_then(|p| p.as_str()) {
                writeln!(output_file, "    {} \\", path)?;
            }
        }
    }

    println!("Generated sparse checkout command in sparse_checkout_command.sh");
    Ok(())
}

use std::{collections::HashMap, fs::File, io::Write};
use serde::{Serialize, Deserialize};
use toml::{Table, Value};
use crate::constants::AGAVE_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    pub path: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}

pub fn extract_packages() -> Result<HashMap<String, PackageInfo>, Box<dyn std::error::Error>> {
    let cargo_toml_path = format!("{}/Cargo.toml", AGAVE_PATH);
    let cargo_toml_content = std::fs::read_to_string(&cargo_toml_path)?;
    let parsed_toml: Value = cargo_toml_content.parse::<Value>()?;

    let workspace_dependencies = parsed_toml
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(|deps| deps.as_table())
        .ok_or("No [workspace.dependencies] section found in Cargo.toml")?;

    let mut package_info_map: HashMap<String, PackageInfo> = HashMap::new();
    
    for (package_name, package_data) in workspace_dependencies {
        if let Some(path) = package_data.get("path") {
            let path_str = path.as_str().unwrap_or("").to_string();
            let (local_deps, local_dev_deps) = get_local_dependencies(path.to_string(), workspace_dependencies);

            package_info_map.insert(package_name.clone(), PackageInfo {
                path: path_str,
                dependencies: local_deps,
                dev_dependencies: local_dev_deps,
            });
        }
    }

    // Save the package information for potential future use
    let mut output_path = File::create("./output/packages_with_path.json")?;
    let json_data = serde_json::to_string_pretty(&package_info_map)?;
    write!(output_path, "{}", json_data)?;

    Ok(package_info_map)
}

fn get_local_dependencies(path: String, workspace_dependencies: &Table) -> (HashMap<String, String>, HashMap<String, String>) {
    let sanitized_path = path.replace(['"', '\\'], "");
    let package_toml_path = format!("{}/{}/Cargo.toml", AGAVE_PATH, sanitized_path);

    let package_toml = match std::fs::read_to_string(package_toml_path.clone()) {
        Ok(content) => content,
        Err(e) => {
            println!("Warning: Could not read {}: {}", package_toml_path, e);
            return (HashMap::new(), HashMap::new());
        }
    };

    let package_toml_parsed: Value = match package_toml.parse() {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Warning: Could not parse {}: {}", package_toml_path, e);
            return (HashMap::new(), HashMap::new());
        }
    };
    
    let package_dependencies = package_toml_parsed
        .get("dependencies")
        .and_then(|deps| deps.as_table());

    let package_dev_dependencies = package_toml_parsed
        .get("dev-dependencies")
        .and_then(|deps| deps.as_table());

    let deps = process_packages(package_dependencies, workspace_dependencies);
    let dev_deps = process_packages(package_dev_dependencies, workspace_dependencies);

    (deps, dev_deps)
}

fn process_packages(
    package_dependencies: Option<&Table>,
    workspace_dependencies: &Table
) -> HashMap<String, String> {
    let mut deps: HashMap<String, String> = HashMap::new();

    if let Some(dependencies) = package_dependencies {
        for (package_name, package_data) in dependencies {
            if package_data.get("workspace").and_then(|w| w.as_bool()).unwrap_or(false) {
                if let Some(workspace_package) = workspace_dependencies.get(package_name) {
                    if let Some(dep_path) = workspace_package.get("path") {
                        let sanitized_dep_path = dep_path.to_string().replace(['"', '\\'], "");

                        // Handle solana-sdk dependencies specially
                        if let Some("sdk") = sanitized_dep_path.split('/').next() {
                            deps.insert("solana-sdk".to_string(), "sdk".to_string());
                        } else {
                            deps.insert(package_name.clone(), sanitized_dep_path.clone());
                        }
                    }
                }
            }
        }
    }

    deps
} 
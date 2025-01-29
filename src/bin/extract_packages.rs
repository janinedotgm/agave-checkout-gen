use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use toml::{Table, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    path: String,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    path: String,
}

fn get_local_dependencies(path: String, workspace_dependencies: &Table) -> (HashMap<String, String>, HashMap<String, String>) {
    println!("checking dependencies for path: {:?}", path);
    let sanitized_path = path.replace(['"', '\\'], "");
    let package_toml_path = format!("./../agave/{}/Cargo.toml", sanitized_path);

    println!("package_toml_path is {:?}", package_toml_path);
    let package_toml = match fs::read_to_string(package_toml_path.clone()) {
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

    let mut deps: HashMap<String, String> = HashMap::new();
    let mut dev_deps: HashMap<String, String> = HashMap::new();

    if let Some(dependencies) = package_dependencies {
        for (package_name, package_data) in dependencies.iter() {
            if package_name.starts_with("solana-") && package_data.get("workspace").and_then(|w| w.as_bool()).unwrap_or(false) {
                if let Some(workspace_package) = workspace_dependencies.get(package_name) {
                    if let Some(dep_path) = workspace_package.get("path") {
                        let sanitized_dep_path = dep_path.to_string().replace(['"', '\\'], "");
                        deps.insert(package_name.clone(), sanitized_dep_path.clone());
                    }
                }
            }
        }
    }

    if let Some(dev_dependencies) = package_dev_dependencies {
        for (package_name, package_data) in dev_dependencies.iter() {
            if package_name.starts_with("solana-") && package_data.get("workspace").and_then(|w| w.as_bool()).unwrap_or(false) {
                if let Some(workspace_package) = workspace_dependencies.get(package_name) {
                    if let Some(dep_path) = workspace_package.get("path") {
                        let sanitized_dep_path = dep_path.to_string().replace(['"', '\\'], "");
                        dev_deps.insert(package_name.clone(), sanitized_dep_path.clone());
                    }
                }
            }
        }
    }

    (deps, dev_deps)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml_path = "./../agave/Cargo.toml";

    let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;

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

            let package_info = PackageInfo {
                path: path_str,
                dependencies: local_deps,
                dev_dependencies: local_dev_deps,
            };

            package_info_map.insert(package_name.clone(), package_info);
        }
    }

    let mut output_file = File::create("./output/packages_with_path.json")?;
    let json_data = serde_json::to_string_pretty(&package_info_map).unwrap();
    write!(output_file, "{}", json_data)?;

    println!("List of packages with `path = ...` written to `packages_with_path.txt`.");

    Ok(())
}
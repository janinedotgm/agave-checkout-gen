use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
};

fn main() {
    let file_content = fs::read_to_string("./output/packages_with_path.json")
        .expect("Failed to read file");
    let dependencies: HashMap<String, Module> =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

    let mut dependency_map: HashMap<String, HashSet<String>> = HashMap::new();

    for (module, module_data) in &dependencies {
        println!("module: {}", module);
        let mut collected_deps = HashSet::new();
        collect_dependencies(module, &dependencies, &mut collected_deps);
        collected_deps.insert(module_data.path.clone()); // Include the module's own path
        dependency_map.insert(module.clone(), collected_deps);
    }

    // Create a HashSet to store unique paths
    let mut unique_paths = HashSet::new();

    // Collect all unique paths
    for paths in dependency_map.values() {
        for path in paths {
            unique_paths.insert(path.clone());
        }
    }

    // Create output file
    let mut output_file = fs::File::create("./output/sparse_checkout_command.sh")
        .expect("Failed to create output file");

    // Write the git command
    writeln!(output_file, "git sparse-checkout set \\").expect("Failed to write to file");

    // Write unique paths to the output file
    for path in unique_paths {
        writeln!(output_file, "    {} \\", path).expect("Failed to write path");
    }

    println!("Generated sparse checkout command in sparse_checkout_command.sh");
}

#[derive(Debug, serde::Deserialize)]
struct Module {
    path: String,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
}

fn collect_dependencies(
    module: &str,
    dependencies: &HashMap<String, Module>,
    collected_deps: &mut HashSet<String>,
) {
    println!("Collecting dependencies for module: {}", module);
    if module.contains(".") {
        println!("Skipping module with name '.' or '..'");
        return;
    }
    if let Some(module_data) = dependencies.get(module) {
        // Add regular dependencies
        for dep in module_data.dependencies.values() {
            if dep.contains("solana-compute-budget-interface") {
                println!("solana-compute-budget-interface found: {}", dep);
            }
            if collected_deps.insert(dep.clone()) {
                println!("Added dependency: {}", dep);
                collect_dependencies(dep, dependencies, collected_deps);
            }
        }

        // Add development dependencies
        for dev_dep in module_data.dev_dependencies.values() {
            println!("Checking dev dependency: {}", dev_dep);
            if dev_dep.contains(".") {
                println!("Skipping dev dependency with name '.' or '..'");
                continue;
            }
            if collected_deps.insert(dev_dep.clone()) {
                println!("Added dev dependency: {}", dev_dep);
                collect_dependencies(dev_dep, dependencies, collected_deps);
            }
        }
    }
}
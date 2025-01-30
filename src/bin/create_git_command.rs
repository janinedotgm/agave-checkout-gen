use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    env,
};

fn main() {
    // Get package name from command line arguments
    let args: Vec<String> = env::args().collect();
    let package_name = args.get(1).expect("Please provide a package name");

    let file_content = fs::read_to_string("./output/packages_with_path.json")
        .expect("Failed to read file");
    let dependencies: HashMap<String, Module> =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Check if the package exists
    if !dependencies.contains_key(package_name) {
        eprintln!("Package '{}' not found", package_name);
        std::process::exit(1);
    }

    let mut collected_deps = HashSet::new();
    collect_dependencies(package_name, &dependencies, &mut collected_deps);
    
    // Add the package's own path
    if let Some(module_data) = dependencies.get(package_name) {
        collected_deps.insert(module_data.path.clone());
    }

    // Create output file
    let mut output_file = fs::File::create("./output/sparse_checkout_command.sh")
        .expect("Failed to create output file");

    // Write the git command
    writeln!(output_file, "git sparse-checkout set \\").expect("Failed to write to file");

    // Write paths to the output file
    for path in collected_deps {
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
    collected_deps.insert(module.replace("solana-", "").to_string());
    if module.contains(".") {
        println!("Skipping module with name '.' or '..'");
        return;
    }
  
    if let Some(module_data) = dependencies.get(module) {
      for (key, value) in &module_data.dependencies {
        println!("Dependency: {} -> {}", key, value);
        } 
        // Add regular dependencies
        let mut c = 0;
        for (key, value) in &module_data.dependencies {
            println!("dep: {}", key);

            c += 1;
            println!("c: {}", c);
            println!("collecting sub dep: {}", key);
            if collected_deps.insert(value.clone()) {
                collect_dependencies(key, dependencies, collected_deps);
            }
        }

        // Add development dependencies
        for (key, value) in &module_data.dev_dependencies {
            if value.contains(".") {
                continue;
            }
            
            if collected_deps.insert(value.clone()) {
                collect_dependencies(key, dependencies, collected_deps);
            }
            
        }
    }
}
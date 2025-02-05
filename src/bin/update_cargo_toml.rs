use std::{fs, fs::File, io::Write};
use toml::Table;
use agave_checkout_gen::constants::AGAVE_PATH;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the original Cargo.toml
    let toml_path = format!("{}/Cargo.toml", AGAVE_PATH);
    let cargo_toml_content = fs::read_to_string(toml_path)?;
    let mut cargo_toml: Table = cargo_toml_content.parse()?;

    let checkout_path = format!("./output/sparse_checkout_command.sh");
    let checkout_command = fs::read_to_string(checkout_path)?;
    let mut checked_out: Vec<String> = checkout_command
      .lines()
      .skip(1)// first line is git checkout command
      .filter(|line| !line.is_empty())
      .map(|line| line.trim().to_string().replace(" \\",""))
      .filter(|line| !line.starts_with("curves"))
      .collect();

    //
    if checkout_command.contains("curve") {
        checked_out.push("curves/*".to_string());
    }

    if let Some(patch_table) = cargo_toml.get_mut("patch") {
        if let Some(crates_io) = patch_table.get_mut("crates-io").and_then(|c| c.as_table_mut()) {
            let patches_to_add: Vec<String> = crates_io
              .iter()
              .filter_map(|(key, value) | {
                  if let Some(path) = value.get("path").and_then(|v| v.as_str()) {

                      let path_dir = path.split('/').next().unwrap_or("");
                      if checked_out.iter().any(|c| c.starts_with(path_dir)) {

                          Some(format!("{} = {}", key, value))
                      } else {
                          None
                      }
                  } else {
                      None
                  }
              }).collect();

            // Create file for members
            let mut patch_file = File::create("./output/patches.toml")?;

            // Write members to file
            writeln!(patch_file, "[patch.crates-io]")?;
            for p in patches_to_add {
                writeln!(patch_file, "{}", p)?;
            }
        }
    }

    // Get the workspace table
    let workspace = cargo_toml.get_mut( "workspace")
        .and_then(|w| w.as_table_mut())
        .ok_or("No [workspace] section found")?;

    // Get the current members array
    let members = workspace.get("members")
        .and_then(|m| m.as_array())
        .ok_or("No members array found")?;

    // Create file for members
    let mut output_file = File::create("./output/members.toml")?;

    // Write members to file
    writeln!(output_file, "members = [")?;
    for member in members {
        let member_str = member.as_str().unwrap();
        if checked_out.iter().any(|folder| member_str == folder && folder != "sdk") {
            writeln!(output_file, "    \"{}\",", member_str)?;
        }
    }
    // Write closing bracket
    writeln!(output_file, "]")?;

    println!("Successfully updated Cargo.toml with available packages.");
    Ok(())
} 
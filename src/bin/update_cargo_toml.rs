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
    println!("{:?}", checked_out);

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
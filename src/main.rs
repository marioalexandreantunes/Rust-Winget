use colored::*;
use figlet_rs::FIGfont;
use std::fs;
use std::process::Command;
use winapi::um::consoleapi::GetConsoleOutputCP;

fn main() {
    // Set UTF-8 encoding (Windows only)
    #[cfg(windows)]
    {
        unsafe {
            if GetConsoleOutputCP() != 65001 {
                // Note: SetConsoleOutputCP is not available in winapi crate
                // You may need to use a different approach or library for this
                println!("Console is not set to UTF-8 (CP 65001)");
            }
        }
    }

    // Define excluded applications
    let excluded_apps = ["BlueStacks", "AutoIt", "Discord"];

    // Show logo
    print_logo();

    // Header
    println!("{}", "=======================================".cyan());
    println!("{}", "  WINGET UPDATE SCRIPT (RUST VERSION)".green());
    println!("{}", "=======================================".cyan());

    // Temporary file path
    let temp_file = dirs::desktop_dir().unwrap().join("winget_list.txt");

    // Get list of upgradable packages
    println!("\n{} Checking for available updates...", "[INFO]".yellow());
    let output = Command::new("winget")
        .args(["list", "--upgrade-available"])
        .output()
        .expect("Failed to execute winget");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Filter the output to remove empty lines and control characters
    let filtered_lines: Vec<&str> = stdout.lines().skip(1).collect();

    // Find the index of the header line
    let header_index = filtered_lines
        .iter()
        .position(|line| line.contains("Name") && line.contains("ID"))
        .unwrap_or(0);

    // Create filtered content starting from the header
    let filtered_content = filtered_lines[header_index..].join("\n");

    fs::write(&temp_file, filtered_content.as_bytes()).expect("Failed to write temp file");

    println!(
        "{} List of upgradable apps saved to: {:?}",
        "[INFO]".yellow(),
        temp_file
    );

    // Process updates
    println!("\n{} Processing updates...\n", "[INFO]".yellow());

    let mut updated_count = 0;
    let mut skipped_count = 0;

    // Read the temporary file to process updates
    let temp_content = fs::read_to_string(&temp_file).expect("Failed to read temp file");

    for line in temp_content.lines().skip(1) {
        if !line.contains(&"winget") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        println!("{} - Processing...", line);

        let appname = parts[0];
        if excluded_apps.contains(&appname) {
            println!(
                "{} {} - Excluded from update",
                "[SKIPPED]".yellow(),
                appname
            );
            skipped_count += 1;
            continue;
        }

        println!("{} {}", "[UPDATING]".green(), appname);
        let status = Command::new("winget")
            .args([
                "update",
                "-q",
                appname,
                "--accept-package-agreements",
                "--accept-source-agreements",
            ])
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("{} {} updated successfully!", "[SUCCESS]".green(), appname);
                updated_count += 1;
            }
            _ => {
                println!("{} Failed to update {}", "[ERROR]".red(), appname);
            }
        }
        println!();
    }

    // Update Rust toolchain
    println!("\n{}", "=======================================".cyan());
    println!("{}", "Updating Rust toolchain...".green());
    println!("{}", "=======================================".cyan());
    Command::new("rustup")
        .arg("update")
        .status()
        .expect("Failed to run rustup update");

    // Clean up temporary file
    println!("\n\n{} Removing temporary file...", "[CLEANING]".yellow());

    match fs::remove_file(&temp_file) {
        Ok(_) => println!("{} File removed successfully!", "[SUCCESS]".green()),
        Err(_) => println!("{} Could not remove file", "[WARNING]".yellow()),
    }

    // Final summary
    println!("\n\n{}", "=======================================".cyan());
    println!("{}", "UPDATE SUMMARY:".green());
    println!("{}", "=======================================".cyan());
    println!("{} {}", "Applications updated:".green(), updated_count);
    println!("{} {}", "Applications skipped:".yellow(), skipped_count);
    println!("{}", "=======================================".cyan());
    println!("{}", "Update process completed!".green());
    println!("{}", "=======================================".cyan());

    println!("\nPress Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn print_logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("winget UPDATE");
    if let Some(ascii_art) = figure {
        println!("{}", ascii_art.to_string().red());
    }
}

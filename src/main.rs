use colored::*;
use figlet_rs::FIGfont;
use std::process::Command;
use winapi::um::consoleapi::GetConsoleOutputCP;

#[derive(Debug)]
struct WingetPackage {
    name: String,
    id: String,
    current_version: String,
    available_version: String,
    #[allow(dead_code)]
    source: String,
}

fn main() {
    // Set UTF-8 encoding (Windows only)
    #[cfg(windows)]
    {
        unsafe {
            if GetConsoleOutputCP() != 65001 {
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

    // Get list of upgradable packages using the new method
    println!("\n{} Checking for available updates...", "[INFO]".yellow());

    match parse_winget_output() {
        Ok(packages) => {
            if packages.is_empty() {
                println!("{} No updates available!", "[INFO]".green());
                return;
            }

            println!(
                "{} Found {} available updates:",
                "[INFO]".green(),
                packages.len()
            );

            // Display available updates
            for package in &packages {
                println!(
                    "  {} {} -> {}",
                    package.name.cyan(),
                    package.current_version.yellow(),
                    package.available_version.green()
                );
            }

            // Process updates
            println!("\n{} Processing updates...\n", "[INFO]".yellow());

            let mut updated_count = 0;
            let mut skipped_count = 0;

            for package in packages {
                println!("{} - Processing...", package.name);

                // Check if app is excluded
                if excluded_apps
                    .iter()
                    .any(|&excluded| package.name.contains(excluded))
                {
                    println!(
                        "{} {} - Excluded from update",
                        "[SKIPPED]".yellow(),
                        package.name
                    );
                    skipped_count += 1;
                    continue;
                }

                println!("{} {}", "[UPDATING]".green(), package.name);
                let status = Command::new("winget")
                    .args([
                        "update",
                        "-q",
                        &package.id, // Use ID instead of name for more reliable updates
                        "--accept-package-agreements",
                        "--accept-source-agreements",
                    ])
                    .status();

                match status {
                    Ok(exit_status) if exit_status.success() => {
                        println!(
                            "{} {} updated successfully!",
                            "[SUCCESS]".green(),
                            package.name
                        );
                        updated_count += 1;
                    }
                    _ => {
                        println!("{} Failed to update {}", "[ERROR]".red(), package.name);
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

            // Final summary
            println!("\n\n{}", "=======================================".cyan());
            println!("{}", "UPDATE SUMMARY:".green());
            println!("{}", "=======================================".cyan());
            println!("{} {}", "Applications updated:".green(), updated_count);
            println!("{} {}", "Applications skipped:".yellow(), skipped_count);
            println!("{}", "=======================================".cyan());
            println!("{}", "Update process completed!".green());
            println!("{}", "=======================================".cyan());
        }
        Err(e) => {
            println!("{} Error getting winget updates: {}", "[ERROR]".red(), e);
        }
    }

    println!("\nPress Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn parse_winget_output() -> Result<Vec<WingetPackage>, Box<dyn std::error::Error>> {
    let output = Command::new("winget").arg("upgrade").output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    let mut packages = Vec::new();

    // Pular cabeçalho e linha separadora
    for line in lines.iter().skip(2) {
        if line.contains("atualizações disponíveis")
            || line.contains("upgrades available")
            || line.trim().is_empty()
        {
            break;
        }

        if let Some(package) = parse_winget_line(line) {
            packages.push(package);
        }
    }

    Ok(packages)
}

fn parse_winget_line(line: &str) -> Option<WingetPackage> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 5 {
        return None;
    }

    let trimmed = line.trim();

    // Encontrar as posições das colunas baseado nos espaços
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_field = false;

    for ch in trimmed.chars() {
        if ch == ' ' {
            if in_field {
                fields.push(current_field.trim().to_string());
                current_field.clear();
                in_field = false;
            }
        } else {
            current_field.push(ch);
            in_field = true;
        }
    }

    if !current_field.trim().is_empty() {
        fields.push(current_field.trim().to_string());
    }

    // Reagrupar campos (nome pode ter espaços)
    if fields.len() >= 5 {
        let source = fields.last()?.clone();
        let available_version = fields[fields.len() - 2].clone();
        let current_version = fields[fields.len() - 3].clone();
        let id = fields[fields.len() - 4].clone();

        // Nome é tudo antes do ID
        let name_parts: Vec<String> = fields[..fields.len() - 4].to_vec();
        let name = name_parts.join(" ");

        return Some(WingetPackage {
            name,
            id,
            current_version,
            available_version,
            source,
        });
    }

    None
}

fn print_logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("winget UPDATE");
    if let Some(ascii_art) = figure {
        println!("{}", ascii_art.to_string().red());
    }
}

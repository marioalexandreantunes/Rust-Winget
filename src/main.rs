use colored::*;
use figlet_rs::FIGfont;
use std::process::Command;
use winapi::um::consoleapi::GetConsoleOutputCP;

/// Representa um pacote retornado por `winget upgrade`.
/// Cada campo corresponde a uma coluna da tabela do winget.
#[derive(Debug)]
struct WingetPackage {
    /// Nome legível do pacote conforme exibido pelo winget.
    name: String,
    /// Identificador único do pacote no winget (ex.: `Microsoft.VisualStudioCode`).
    id: String,
    /// Versão atualmente instalada no sistema.
    current_version: String,
    /// Versão disponível para atualização.
    available_version: String,
    #[allow(dead_code)]
    /// Fonte ou repositório do pacote (ex.: `winget` ou `msstore`).
    source: String,
}

/// Ponto de entrada principal do programa.
/// - Mostra o logo em ASCII.
/// - Obtém e exibe a lista de atualizações disponíveis.
/// - Atualiza os pacotes não excluídos e, ao final, executa `rustup update`.
/// - Exibe um resumo e aguarda Enter antes de encerrar.
///
/// Segurança:
/// Contém um bloco `unsafe` limitado para ler o code page atual do console via `GetConsoleOutputCP` em Windows.
fn main() {
    // Configura o console para UTF-8 no Windows
    #[cfg(windows)]
    {
        unsafe {
            if GetConsoleOutputCP() != 65001 {
                println!("Console is not set to UTF-8 (CP 65001)");
            }
        }
    }

    // Define aplicativos a serem excluídos
    // Discord não atualiza pelo winget
    // Autoit quero manter a versão que tenho (v3.3.14.5 x86)
    let excluded_apps = ["AutoIt", "Discord"];

    // Exibe o logo
    print_logo();

    // Cabeçalho
    println!("{}", "=======================================".cyan());
    println!("{}", "  WINGET UPDATE SCRIPT (RUST VERSION)".green());
    println!("{}", "=======================================".cyan());

    // Obtém a lista de pacotes atualizáveis usando o novo método
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

            // Exibe as atualizações disponíveis
            for package in &packages {
                println!(
                    "  {} {} -> {}",
                    package.name.cyan(),
                    package.current_version.yellow(),
                    package.available_version.green()
                );
            }

            // Processa atualizações
            println!("\n{} Processing updates...\n", "[INFO]".yellow());

            let mut updated_count = 0;
            let mut skipped_count = 0;

            for package in packages {
                println!("{} - Processando...", package.name);

                // Verifica se o app está excluído
                if excluded_apps
                    .iter()
                    .any(|&excluded| package.name.contains(excluded))
                {
                    println!(
                        "{} {} - Excluído da atualização",
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
                        &package.id, // Usa ID em vez do nome para atualizações mais confiáveis
                        "--accept-package-agreements",
                        "--accept-source-agreements",
                    ])
                    .status();

                match status {
                    Ok(exit_status) if exit_status.success() => {
                        println!(
                            "{} {} atualizado com sucesso!",
                            "[SUCCESS]".green(),
                            package.name
                        );
                        updated_count += 1;
                    }
                    _ => {
                        println!("{} Falha ao atualizar {}", "[ERROR]".red(), package.name);
                    }
                }
                println!();
            }

            // Atualiza Rust toolchain
            println!("\n{}", "=======================================".cyan());
            println!("{}", "Atualizando Rust toolchain...".green());
            println!("{}", "=======================================".cyan());
            Command::new("rustup")
                .arg("update")
                .status()
                .expect("Falha ao executar rustup update");

            // Resumo final
            println!("\n\n{}", "=======================================".cyan());
            println!("{}", "RESUMO DE ATUALIZAÇÃO:".green());
            println!("{}", "=======================================".cyan());
            println!("{} {}", "Aplicativos atualizados:".green(), updated_count);
            println!("{} {}", "Aplicativos ignorados:".yellow(), skipped_count);
            println!("{}", "=======================================".cyan());
            println!("{}", "Processo de atualização concluído!".green());
            println!("{}", "=======================================".cyan());
        }
        Err(e) => {
            println!(
                "{} Erro ao obter atualizações winget: {}",
                "[ERROR]".red(),
                e
            );
        }
    }

    println!("\nPress Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// Executa `winget upgrade` e converte a tabela da saída em uma lista de pacotes.
/// Interrompe o parse quando encontra uma linha informativa (como
/// "available upgrades"/"upgrades available") ou uma linha em branco.
///
/// Retorna:
/// - `Ok(Vec<WingetPackage>)` em caso de sucesso.
/// - `Err(...)` se o comando `winget` falhar ou a saída não puder ser lida.
fn parse_winget_output() -> Result<Vec<WingetPackage>, Box<dyn std::error::Error>> {
    let output = Command::new("winget").arg("upgrade").output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    let mut packages = Vec::new();

    // Pula o cabeçalho e linha separadora
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

/// Faz o parse de uma única linha da tabela de `winget upgrade`.
/// Assume as colunas: Nome, Id, Versão Atual, Versão Disponível e Fonte.
/// O nome pode conter espaços; por isso os últimos quatro campos são lidos de trás para frente.
///
/// Parâmetros:
/// - `line`: linha bruta de saída do comando `winget upgrade`.
///
/// Retorna:
/// - `Some(WingetPackage)` se o parse for bem-sucedido.
/// - `None` se a linha não tiver colunas suficientes ou formato inesperado.
fn parse_winget_line(line: &str) -> Option<WingetPackage> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 5 {
        return None;
    }

    let trimmed = line.trim();

    // Encontra as posições das colunas baseando-se nos espaços
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

    // Reagrupar campos (o nome pode ter espaços)
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

/// Imprime o logo ASCII "winget UPDATE" usando a fonte padrão do `figlet_rs`,
/// colorido em vermelho.
///
/// Nota: em caso de falha ao carregar a fonte padrão, o programa entra em panic via `unwrap()`.
fn print_logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("winget UPDATE");
    if let Some(ascii_art) = figure {
        println!("{}", ascii_art.to_string().red());
    }
}

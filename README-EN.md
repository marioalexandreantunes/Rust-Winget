# winget_update

Small Windows utility written in Rust that automates checking and installing updates using winget. The program:

- Runs `winget upgrade` to get the table of packages with updates available
- Parses the output to extract Name, Id, current version and available version
- Shows the found list and, for each non-excluded package, runs `winget update -q <ID> --accept-package-agreements --accept-source-agreements`
- At the end, runs `rustup update` to update the Rust toolchain
- Prints a summary of what was updated and what was ignored

The application displays a small ASCII logo and uses colors in the terminal to facilitate reading.

## How it works

1. Right at startup, the program checks if the console code page is UTF-8 (CP 65001) and only informs if it's not.
2. Gets the upgrade list with `winget upgrade` and parses the table lines.
3. Displays the found packages (name and versions).
4. For each package:
   - If the name contains any item from the exclusion list (array `excluded_apps`), the package is skipped.
   - Otherwise, runs `winget update` using the package ID (more reliable than name) with automatic acceptance of terms.
5. At the end, calls `rustup update` and prints a summary with count of updated and ignored packages.

Parsing notes:
- The code ignores the first two lines of tabular output and stops reading when it finds the footer line containing "atualizações disponíveis" (pt-BR) or "upgrades available" (en), or a blank line.
- The application name can contain spaces; the parser regroups the fields to correctly extract `name`, `id`, `current_version`, `available_version` and `source`.

## Requirements

- Windows 10/11 (winget is only available on Windows).
- winget installed and available in PATH.
- To compile from source: Rust (cargo) installed.
- For the final step to work: `rustup` installed (otherwise the call to `rustup update` will fail).

## Dependencies (Cargo.toml)

- Runtime:
  - `colored` — colors in terminal.
  - `figlet-rs` — ASCII logo generation.
  - `winapi` (feature `consoleapi`) — console code page query.
- Build (Windows):
  - `winres` — includes Windows resources (icon) in the executable.
- Note: `dirs` is listed but not used in current code and can be removed if desired.

## Build and execution

From the repository root:

- Compile in release (recommended):

  ```bash
  cargo build --release
  ```

- Run (via cargo or generated binary):

  ```bash
  cargo run --release
  ```

The release executable will be in `target/release/`.

## Usage

- Run the binary. The list of packages with updates available will be displayed, then the update process.
- You may need to run with elevated privileges (Administrator) for some installations to be completed by winget.

## Configuration

- Exclusion list: defined in `src/main.rs` in the `excluded_apps` array.
  - Default: `["AutoIt", "Discord"]`.
  - The check is by substring in the package name (if the name contains the text). Adjust as needed and recompile.
- Terms acceptance: the program calls `winget update` with `--accept-package-agreements` and `--accept-source-agreements`.

## Windows Resources (icon)

- On Windows, `build.rs` uses `winres` to embed the `ICON.ico` icon in the executable. Make sure to keep the `ICON.ico` file in the project root for the icon to be applied.

## Limitations and possible improvements

- The output analysis depends on the tabular format of `winget upgrade` and may break if formatting changes in future winget versions.
- Currently stops at the summary line (in Portuguese/English). If using another language, adjust the keywords in the code.
- Possible improvements:
  - Make the exclusion list configurable via CLI/file/env (e.g., with `clap`).
  - Add "dry-run" mode (simulate without applying) and logs to file.
  - Make the parser more resilient or use a structured format if/when available.

## Project metadata

- Name: `winget_update`
- Version: 0.1.0
- Release profile: optimizations for size, LTO, `panic = "abort"`, debug symbols removed (see `Cargo.toml`).
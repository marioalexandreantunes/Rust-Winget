# winget_update

A small Windows-focused Rust utility that automates checking for and installing available updates using winget. The program:

- Calls `winget list --upgrade-available` to obtain a list of packages with available updates.
- Filters and saves the result to a temporary file on the desktop (`winget_list.txt`).
- Iterates the list and runs `winget update` for each package (skipping apps listed in the excluded list).
- Updates the Rust toolchain via `rustup update` after processing application updates.
- Removes the temporary file and prints a summary of updated and skipped applications.

This repository contains a compact command-line program with colored console output and a small ASCII logo.

## Features

- Uses winget to detect and install updates automatically.
- Skips user-specified applications (configured in source).
- Saves intermediate data to a desktop temporary file so you can inspect the list before updates.
- Prints a summary of how many applications were updated or skipped.
- Updates the Rust toolchain at the end.

## Requirements

- Windows 10/11 (winget is only available on Windows)
- winget installed and available in PATH
- Rust toolchain (to build from source) with `cargo` and `rustup`

## Dependencies

The project uses these crates (declared in Cargo.toml):

- colored = "3.0.0" — colored terminal output
- dirs = "6.0.0" — to find the Desktop path
- winapi = { version = "0.3.9", features = ["consoleapi"] } — to query console code page on Windows

Build-time dependency for Windows resources (optional):

- winres = "0.1" (configured as a build-dependency for windows target)

## Build & Run

From the repository root:

1. Build in release mode (recommended):

   cargo build --release

2. Run the compiled binary (or run directly with cargo):

   cargo run --release

The release build will be placed under `target/release/`.

Note: .cargo/config.toml contains `-C target-feature=+crt-static`. This attempts to link the CRT statically for the provided Windows targets. If you do not want static CRT linking or encounter linker issues, remove or edit that flag.

## Configuration

- Excluded applications are hard-coded in `src/main.rs` in the `excluded_apps` array. By default:

  let excluded_apps = ["BlueStacks", "AutoIt", "Discord"];

  Modify this list and rebuild to change which applications are skipped.

- Temporary file path: the program writes `winget_list.txt` to your Desktop directory. The file is removed at the end, but the file remains if the remove fails for any reason.

- Header detection: the code currently looks for a header line that contains the Portuguese column names `"Nome"` and `"ID"` to locate the start of the table. If your winget locale prints headers in English or another language (for example, `Name` and `Id`), the header detection may not work correctly. Consider updating this check in `src/main.rs` to match your locale or implement a more robust parsing strategy.

## Example behavior

- The program prints a colored logo and an information header, then runs `winget list --upgrade-available` and prints each discovered line.
- For each package that looks like a winget-managed entry, it runs `winget update -q <package> --accept-package-agreements --accept-source-agreements`.
- A short summary is printed at the end with counts of updated and skipped applications.

## Troubleshooting

- "Failed to execute winget": ensure `winget` is installed and available in PATH. Open a PowerShell or CMD and run `winget --version` to validate.
- If updates fail for specific packages, try running `winget upgrade <package>` manually to inspect error messages.
- If the program cannot find your Desktop directory, the `dirs` crate may not return a value; the code uses `dirs::desktop_dir().unwrap()` which will panic if no Desktop path is available. You can change this behavior to provide a fallback path.
- If the header detection fails due to localization, update the check that searches for `"Nome"` and `"ID"` in `src/main.rs`.

## Suggestions / Improvements

- Make the excluded apps configurable via a CLI flag, config file, or environment variable instead of hard-coding.
- Make header detection locale-agnostic by parsing columns by position or using winget in a machine-parseable format (if available) or using `--manifest` / API output.
- Add logging to a file and/or a dry-run mode to preview changes without performing updates.
- Add argument parsing (e.g., using clap) to control behavior at runtime: dry-run, exclude-list file, path for temporary file, skip rustup update, etc.

## Project Metadata

- Package name: `winget_update` (see Cargo.toml)
- Version: 0.1.0
- Cargo profile (release) options: optimized for size and LTO enabled (see Cargo.toml profile.release section)

## License

No license file is provided. If you intend to publish or share this project, consider adding a license (for example, MIT or Apache-2.0) and adding a LICENSE file.

## Contributing

Pull requests and issues are welcome. For suggestions that change default behavior, please document configuration and any new flags in README.

---

If you like, I can also:

- Add a sample CLI with argument parsing using clap and replace the hard-coded excluded list with a CLI or config option.
- Update the header detection to handle English/Portuguese automatically or parse the winget output more robustly.
- Add a LICENSE file.

Which of these would you like me to do next?
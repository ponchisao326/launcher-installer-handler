# Minecraft Launcher Installer Library

This Rust library provides a streamlined way to extract ZIP-based updates or installations for Minecraft launchers and similar applications, with optional features for cleanup and system restart.

## Features

- **Flexible Extraction:** Extracts ZIP files to any specified directory, ensuring all folders are created as needed.
- **Automatic Cleanup:** Optionally deletes the ZIP file after extraction.
- **System Restart:** Optionally restarts the system after installing or updating.
- **Easy Integration:** Designed for use within Rust-based Minecraft launchers or any application requiring robust ZIP extraction and post-update actions.

## Example Usage

```rust
use my_installer_library::InstallerOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create installer options
    let mut options = InstallerOptions::new(
        "path/to/update.zip".to_string(),
        "path/to/extract/".to_string(),
        true,  // Delete ZIP after extraction
        false  // Do not restart after extraction
    );

    // Perform extraction (and optional cleanup/restart)
    options.extract()?;

    Ok(())
}
```

## Main Structure

- `InstallerOptions`:
    - `zip_path` — Path to the ZIP file to extract.
    - `extract_to` — Directory to extract the contents into.
    - `delete_zip_after` — If `true`, deletes the ZIP after extraction.
    - `restart_after_bool` — If `true`, restarts the system after extraction.

### Example

```rust
let mut installer = InstallerOptions::new(
    "updates/launcher_update.zip".to_string(),
    "launcher/".to_string(),
    true,
    true
);
installer.extract()?;
```

## Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
zip = "0.6"
system-shutdown = "3.0"
launcher-request-handler = "your_version_here"
```

## Motivation

Originally created for my own Minecraft launcher to automate the update and installation process. Published to help others implement safe, user-friendly update flows in Rust apps.

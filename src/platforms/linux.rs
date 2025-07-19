#![cfg(target_os = "linux")]
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Get configuration paths in order of preference
pub fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Try XDG_CONFIG_HOME first (most standard)
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        paths.push(
            PathBuf::from(xdg_config)
                .join("qmk-notifier")
                .join("config.toml"),
        );
    }

    // Try home directory paths as fallback
    if let Some(home) = dirs::home_dir() {
        paths.push(
            home.join(".config")
                .join("qmk-notifier")
                .join("config.toml"),
        );
    }

    // Try system-wide config as last resort
    paths.push(PathBuf::from("/etc/qmk-notifier/config.toml"));

    paths
}

// Update udev rules with vendor and product IDs
pub fn update_udev_rules(
    vendor_id: String,
    product_id: String,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let rules_path = Path::new("/etc/udev/rules.d/99-qmkonnect.rules");

    if verbose {
        println!("Updating udev rules at {}", rules_path.display());
    }

    // Check if the rules file exists
    if !rules_path.exists() {
        return Err(format!("Udev rules file not found at {}. This operation may need to be run by root during package installation.", rules_path.display()).into());
    }

    // Format vendor_id and product_id for udev rules (lowercase, no 0x prefix)
    let vendor_id_formatted = vendor_id.trim_start_matches("0x").to_lowercase();
    let product_id_formatted = product_id.trim_start_matches("0x").to_lowercase();

    if verbose {
        println!(
            "Using formatted vendor_id: {} and product_id: {}",
            vendor_id_formatted, product_id_formatted
        );
    }

    // Read the rules file
    let rules_content = fs::read_to_string(rules_path)?;

    // Update the vendor and product IDs in the rules file
    let updated_rules = rules_content
        .replace(
            &format!(
                "ATTRS{{idVendor}}==\"{}\"",
                extract_attr_value(&rules_content, "idVendor")?
            ),
            &format!("ATTRS{{idVendor}}==\"{}\"", vendor_id_formatted),
        )
        .replace(
            &format!(
                "ATTRS{{idProduct}}==\"{}\"",
                extract_attr_value(&rules_content, "idProduct")?
            ),
            &format!("ATTRS{{idProduct}}==\"{}\"", product_id_formatted),
        );

    // Write back to the file with sudo
    let temp_path = "/tmp/99-qmkonnect.rules.tmp";
    fs::write(temp_path, updated_rules)?;

    let output = Command::new("sudo")
        .arg("mv")
        .arg(temp_path)
        .arg(rules_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to update udev rules: {}", stderr).into());
    }

    Ok(())
}

// Reload udev rules
pub fn reload_udev_rules() -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("udevadm")
        .arg("control")
        .arg("--reload-rules")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to reload udev rules: {}", stderr).into());
    }

    Ok(())
}

// Helper function to extract attribute values from udev rules
fn extract_attr_value(content: &str, attr: &str) -> Result<String, Box<dyn Error>> {
    let attr_pattern = format!("ATTRS{{{}}}==\"", attr);

    for line in content.lines() {
        if let Some(start_idx) = line.find(&attr_pattern) {
            let start_idx = start_idx + attr_pattern.len();
            if let Some(end_idx) = line[start_idx..].find('\"') {
                return Ok(line[start_idx..start_idx + end_idx].to_string());
            }
        }
    }

    Err(format!("Could not find {} in udev rules", attr).into())
}

// For creating configuration directory
pub fn create_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config).join("qmk-notifier")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".config").join("qmk-notifier")
    } else {
        return Err("Could not determine configuration directory".into());
    };

    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

// Helper to convert decimal vendor_id/product_id back to hex format for udev rules
pub fn decimal_to_hex(decimal: u16) -> String {
    format!("{:04x}", decimal)
}

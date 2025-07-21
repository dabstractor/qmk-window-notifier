pub mod config;
pub mod notifier;
pub mod types;

use std::error::Error;
use std::fs;
use std::path::Path;

// Define the Config struct
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    // Use u16 for the IDs
    pub vendor_id: u16,
    pub product_id: u16,
    // Add any other fields your config needs
}

pub fn parse_config(config_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    // No need to normalize or validate - TOML parser handles it

    Ok(config)
}

// Add a function to create a default config file
pub fn create_default_config(config_path: &Path) -> Result<(), Box<dyn Error>> {
    if config_path.exists() {
        println!("Configuration already exists at: {}", config_path.display());
        return Ok(());
    }

    // Create default config with hex values (no quotes)
    let default_config = r#"# QMKonnect Configuration

# Your QMK keyboard's vendor ID (in hex)
vendor_id = 0xfeed

# Your QMK keyboard's product ID (in hex)
product_id = 0x0000

# Add any other configuration options here
"#;

    // Make sure the directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the config file
    fs::write(config_path, default_config)?;

    println!(
        "Configuration created successfully at: {}",
        config_path.display()
    );
    println!("Note: You'll need to reload configuration with 'qmkonnect -r' to update udev rules.");

    Ok(())
}

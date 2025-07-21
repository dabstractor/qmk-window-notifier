use crate::core::Config;
use std::error::Error;
use std::path::PathBuf;

/// Unified configuration management trait
#[allow(dead_code)]
pub trait ConfigManager {
    /// Get configuration file paths in order of preference
    fn get_config_paths(&self) -> Vec<PathBuf>;
    
    /// Create configuration directory
    fn create_config_dir(&self) -> Result<PathBuf, Box<dyn Error>>;
    
    /// Find existing configuration file
    fn find_config_file(&self) -> Result<PathBuf, Box<dyn Error>> {
        let config_paths = self.get_config_paths();
        
        // Try each path in order
        for path in config_paths {
            if path.exists() {
                return Ok(path);
            }
        }
        
        Err("No configuration file found in any of the expected locations".into())
    }
    
    /// Load configuration from file
    fn load_config(&self) -> Result<Config, Box<dyn Error>> {
        let config_path = self.find_config_file()?;
        crate::core::parse_config(&config_path)
    }
    
    /// Create default configuration file
    fn create_default_config(&self) -> Result<PathBuf, Box<dyn Error>> {
        let config_dir = self.create_config_dir()?;
        let config_path = config_dir.join("config.toml");
        crate::core::create_default_config(&config_path)?;
        Ok(config_path)
    }
    
    /// Platform-specific configuration update (e.g., udev rules on Linux)
    fn update_platform_config(&self, config: &Config, verbose: bool) -> Result<(), Box<dyn Error>>;
}

/// Platform-specific configuration manager implementations
#[cfg(target_os = "linux")]
pub struct LinuxConfigManager;

#[cfg(target_os = "windows")]
pub struct WindowsConfigManager;

#[cfg(target_os = "macos")]
pub struct MacOSConfigManager;

// Linux implementation
#[cfg(target_os = "linux")]
impl ConfigManager for LinuxConfigManager {
    fn get_config_paths(&self) -> Vec<PathBuf> {
        crate::platforms::get_config_paths()
    }
    
    fn create_config_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        crate::platforms::create_config_dir()
    }
    
    fn update_platform_config(&self, config: &Config, verbose: bool) -> Result<(), Box<dyn Error>> {
        // Convert the numeric values back to hex strings for udev rules
        let vendor_id_hex = crate::platforms::decimal_to_hex(config.vendor_id);
        let product_id_hex = crate::platforms::decimal_to_hex(config.product_id);

        if let Err(e) = crate::platforms::update_udev_rules(vendor_id_hex, product_id_hex, verbose) {
            if verbose {
                println!("Warning: Could not update udev rules: {}", e);
            }
        }

        if let Err(e) = crate::platforms::reload_udev_rules() {
            if verbose {
                println!("Warning: Could not reload udev rules: {}", e);
            }
        }
        
        Ok(())
    }
}

// Windows implementation
#[cfg(target_os = "windows")]
impl ConfigManager for WindowsConfigManager {
    fn get_config_paths(&self) -> Vec<PathBuf> {
        crate::platforms::get_config_paths()
    }
    
    fn create_config_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        crate::platforms::create_config_dir()
    }
    
    fn update_platform_config(&self, _config: &Config, _verbose: bool) -> Result<(), Box<dyn Error>> {
        // Windows doesn't need platform-specific configuration updates
        Ok(())
    }
}

// macOS implementation
#[cfg(target_os = "macos")]
impl ConfigManager for MacOSConfigManager {
    fn get_config_paths(&self) -> Vec<PathBuf> {
        crate::platforms::get_config_paths()
    }
    
    fn create_config_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        crate::platforms::create_config_dir()
    }
    
    fn update_platform_config(&self, _config: &Config, _verbose: bool) -> Result<(), Box<dyn Error>> {
        // macOS doesn't need platform-specific configuration updates
        Ok(())
    }
}

/// Create platform-specific configuration manager
#[allow(dead_code)]
pub fn create_config_manager() -> Box<dyn ConfigManager> {
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxConfigManager)
    }
    
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsConfigManager)
    }
    
    #[cfg(target_os = "macos")]
    {
        Box::new(MacOSConfigManager)
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        compile_error!("Unsupported platform for configuration management")
    }
}
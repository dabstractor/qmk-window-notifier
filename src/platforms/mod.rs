mod hyprland;
mod linux;
#[cfg(target_os = "macos")]
mod macos;

// Define WindowMonitor trait
#[cfg(not(all(target_os = "linux", feature = "hyprland")))]
pub trait WindowMonitor: Send {
    fn platform_name(&self) -> &str;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    // Add attribute to suppress dead code warning
    #[allow(dead_code)]
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation
        Ok(())
    }
}
#[cfg(all(target_os = "linux", feature = "hyprland"))]
pub trait WindowMonitor {
    fn platform_name(&self) -> &str;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    // Add attribute to suppress dead code warning
    #[allow(dead_code)]
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation
        Ok(())
    }
}

// Export Linux module's functions
#[cfg(target_os = "linux")]
pub use linux::*;

use std::error::Error;

// Return a platform-specific monitor implementation
pub fn create_monitor(verbose: bool) -> Result<Box<dyn WindowMonitor>, Box<dyn Error>> {
    // Platform-specific implementations
    #[cfg(all(target_os = "linux", feature = "hyprland"))]
    {
        use hyprland::HyprlandMonitor;
        return Ok(Box::new(HyprlandMonitor::new(verbose)));
    }

    #[cfg(target_os = "macos")]
    {
        use macos::MacOSMonitor;
        return Ok(Box::new(MacOSMonitor::new(verbose)));
    }

    // Fix unreachable code warning by removing the 'return' keywords above
    // and using a more idiomatic approach
    #[cfg(not(any(all(target_os = "linux", feature = "hyprland"), target_os = "macos")))]
    Err("No suitable monitor for this platform".into())
}

// Get configuration paths based on current platform
pub fn get_config_paths() -> Vec<std::path::PathBuf> {
    #[cfg(target_os = "linux")]
    return linux::get_config_paths();

    #[cfg(target_os = "macos")]
    return Vec::new(); // Placeholder for macOS

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    return Vec::new(); // Default for other platforms
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    pub struct MockWindowMonitor {
        platform_name: String,
        start_called: bool,
        stop_called: bool,
    }

    impl MockWindowMonitor {
        pub fn new(platform_name: &str) -> Self {
            Self {
                platform_name: platform_name.to_string(),
                start_called: false,
                stop_called: false,
            }
        }

        pub fn was_start_called(&self) -> bool {
            self.start_called
        }

        pub fn was_stop_called(&self) -> bool {
            self.stop_called
        }
    }

    impl WindowMonitor for MockWindowMonitor {
        fn platform_name(&self) -> &str {
            &self.platform_name
        }

        fn start(&mut self) -> Result<(), Box<dyn Error>> {
            self.start_called = true;
            Ok(())
        }

        fn stop(&mut self) -> Result<(), Box<dyn Error>> {
            self.stop_called = true;
            Ok(())
        }
    }

    #[test]
    fn test_window_monitor_implementation() {
        let mut monitor = MockWindowMonitor::new("Mock Platform");

        // Test platform_name
        assert_eq!(monitor.platform_name(), "Mock Platform");

        // Test start
        let result = monitor.start();
        assert!(result.is_ok());
        assert!(monitor.was_start_called());

        // Test stop
        let result = monitor.stop();
        assert!(result.is_ok());
        assert!(monitor.was_stop_called());
    }
}

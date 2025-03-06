mod hyprland;
mod macos;

use std::error::Error;

pub trait WindowMonitor {
    fn platform_name(&self) -> &str;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    #[allow(dead_code)]
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
}

pub fn create_monitor(verbose: bool) -> Result<Box<dyn WindowMonitor>, Box<dyn Error>> {
    // Use a let-else pattern to avoid multiple returns with early detection
    #[cfg(target_os = "macos")]
    {
        return Ok(Box::new(macos::MacOSMonitor::new(verbose)));
    }

    #[cfg(all(target_os = "linux", feature = "hyprland"))]
    {
        // For Linux, we need to detect the specific desktop environment
        if hyprland::is_hyprland_running() {
            return Ok(Box::new(hyprland::HyprlandMonitor::new(verbose)));
        } else {
            return Err("Hyprland is not running".into());
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        // If we can't detect a supported platform
        Err("Unsupported platform or desktop environment".into())
    }
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

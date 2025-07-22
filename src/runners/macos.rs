#![cfg(target_os = "macos")]

use crate::platforms;
use crate::runners::PlatformRunner;
use std::error::Error;
use std::process;

pub struct MacOSRunner {
    verbose: bool,
}

impl MacOSRunner {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl PlatformRunner for MacOSRunner {
    fn run(&mut self, _args: &[String]) -> Result<(), Box<dyn Error>> {
        let mut monitor = platforms::create_monitor(self.verbose)?;

        println!("QMKonnect started");
        if self.verbose {
            println!("Verbose logging enabled");
            println!("Using platform: {}", monitor.platform_name());
        }

        // Set up signal handling for immediate exit
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down...");
            // Force immediate exit - no waiting or additional complexity
            process::exit(0);
        })?;

        // Start the monitor in a separate thread for macOS
        let monitor_thread = std::thread::spawn(move || {
            if let Err(e) = monitor.start() {
                eprintln!("Monitor error: {}", e);
            }
        });

        // Setup tray icon for macOS - this will block until the user quits
        crate::tray::setup_tray();

        if self.verbose {
            println!("System tray closed, shutting down...");
        }

        // The tray has closed, so we can exit
        // The monitor thread will be cleaned up when the process exits
        drop(monitor_thread);

        Ok(())
    }
}
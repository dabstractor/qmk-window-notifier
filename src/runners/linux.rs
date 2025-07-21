#![cfg(target_os = "linux")]

use crate::platforms;
use crate::runners::PlatformRunner;
use std::error::Error;
use std::process;

pub struct LinuxRunner {
    verbose: bool,
}

impl LinuxRunner {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl PlatformRunner for LinuxRunner {
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

        // For Hyprland, start the monitor on the main thread
        #[cfg(all(target_os = "linux", feature = "hyprland"))]
        {
            if let Err(e) = monitor.start() {
                eprintln!("Monitor error: {}", e);
                return Err(e);
            }
        }

        // For non-Hyprland Linux, start the monitor in a separate thread
        #[cfg(all(target_os = "linux", not(feature = "hyprland")))]
        {
            use std::thread;
            
            let monitor_thread = thread::spawn(move || {
                if let Err(e) = monitor.start() {
                    eprintln!("Monitor error: {}", e);
                }
            });

            // Setup tray icon for non-Hyprland Linux
            crate::tray::setup_tray();

            if self.verbose {
                println!("System tray icon initialized");
            }

            // Join the monitor thread
            if let Err(e) = monitor_thread.join() {
                eprintln!("Error joining Monitor thread: {:?}", e);
            }
        }

        // If we reach here, the monitor stopped on its own
        println!("Monitor stopped, exiting.");

        Ok(())
    }
}
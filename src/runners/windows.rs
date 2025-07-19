#![cfg(target_os = "windows")]


use crate::platforms;
use crate::runners::PlatformRunner;
use crate::service;
use crate::tray;
use log::{error, info};
use single_instance::SingleInstance;
use std::error::Error;
use std::process;

pub struct WindowsRunner {
    verbose: bool,
}

impl WindowsRunner {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    fn is_already_running() -> Result<bool, Box<dyn Error>> {
        // Create a single instance using a unique app ID
        // This will create a named mutex under the hood
        static mut INSTANCE: Option<SingleInstance> = None;

        let instance = SingleInstance::new("qmkonnect-app-id").map_err(|e| -> Box<dyn Error> {
            format!("Failed to create single instance: {}", e).into()
        })?;

        // Check if this is the first instance
        if !instance.is_single() {
            // Another instance is already running
            return Ok(true);
        }

        // Store the instance to keep it alive for the duration of the program
        unsafe {
            INSTANCE = Some(instance);
        }

        // This is the first/only instance
        Ok(false)
    }

    fn run_console_mode(&self) -> Result<(), Box<dyn Error>> {
        // This runs the original console-based logic for Windows debugging
        println!("Creating Windows monitor...");
        let monitor = platforms::create_monitor(self.verbose)?;

        println!("QMKonnect started in console mode");
        if self.verbose {
            println!("Verbose logging enabled");
            println!("Using platform: {}", monitor.platform_name());
        }

        // Set up signal handling for immediate exit
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down...");
            process::exit(0);
        })?;

        // Start the monitor
        println!("Starting Windows monitor...");
        let mut monitor = monitor;
        if let Err(e) = monitor.start() {
            eprintln!("Failed to start Windows monitor: {}", e);
            return Err(e);
        }

        if self.verbose {
            println!("Windows monitor started successfully");
        }

        // Keep the console app running
        println!("Press Ctrl+C to exit...");
        println!("Now switch between different applications to test window detection...");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn run_tray_app(&self) -> Result<(), Box<dyn Error>> {
        // Check for existing instance (singleton)
        if Self::is_already_running()? {
            if self.verbose {
                println!("Another instance is already running, exiting");
            }
            info!("Another instance is already running, exiting");
            return Ok(());
        }

        if self.verbose {
            println!("No other instance detected, starting application");
        }
        info!("Starting QMKonnect as tray application");

        // Create the monitor
        let monitor = platforms::create_monitor(self.verbose)?;

        if self.verbose {
            info!("Using platform: {}", monitor.platform_name());
        }

        // On Windows, start the monitor before setting up the tray (like original working code)
        let mut monitor = monitor;
        if let Err(e) = monitor.start() {
            error!("Failed to start Windows monitor: {}", e);
            return Err(e);
        }
        if self.verbose {
            println!("Windows monitor started successfully");
        }

        // Setup tray icon - this will block until the user quits
        tray::setup_tray();

        // If we reach here, the tray was closed
        info!("Tray application shutting down");

        // The monitor thread will be terminated when the process exits
        // We don't need to explicitly join it since the tray exit means the user wants to quit

        Ok(())
    }
}

impl PlatformRunner for WindowsRunner {
    fn run(&mut self, args: &[String]) -> Result<(), Box<dyn Error>> {
        // Windows service-specific arguments
        if args.iter().any(|arg| arg == "--service") {
            info!("Starting as Windows service");
            return service::run_service();
        }

        if args.iter().any(|arg| arg == "--install-service") {
            return service::install_service();
        }

        if args.iter().any(|arg| arg == "--uninstall-service") {
            return service::uninstall_service();
        }

        if args.iter().any(|arg| arg == "--start-service") {
            return service::start_service();
        }

        if args.iter().any(|arg| arg == "--stop-service") {
            return service::stop_service();
        }

        // Check if running as tray app
        if args.iter().any(|arg| arg == "--tray-app") {
            info!("Starting as tray application");
            return self.run_tray_app();
        }

        // Check if running in console mode (for debugging)
        if args.iter().any(|arg| arg == "--console") {
            // Allocate a console for this GUI app so we can see output
            unsafe {
                use windows::Win32::System::Console::AllocConsole;
                let _ = AllocConsole();
            }
            return self.run_console_mode();
        }

        // Default behavior on Windows: run as tray app
        info!("Starting as tray application (default)");
        self.run_tray_app()
    }
}
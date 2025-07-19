#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod core;
mod platforms;
mod tray;

#[cfg(target_os = "windows")]
mod service;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process;

#[cfg(target_os = "windows")]
use log::{error, info};

#[cfg(target_os = "windows")]
fn init_logging() -> Result<(), Box<dyn Error>> {
    // Try to initialize Windows Event Log first
    match eventlog::init("QMK Window Notifier", log::Level::Info) {
        Ok(()) => {
            info!("Windows Event Log initialized");
            Ok(())
        }
        Err(e) => {
            // Fallback to console logging if event log fails
            env_logger::init();
            eprintln!("Failed to initialize Windows Event Log, using console: {}", e);
            Ok(())
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn init_logging() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    Ok(())
}

fn main() {
    // Initialize logging first
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        process::exit(1);
    }

    if let Err(e) = run() {
        #[cfg(target_os = "windows")]
        error!("Application error: {}", e);
        #[cfg(not(target_os = "windows"))]
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let verbose = args.iter().any(|arg| arg == "-v" || arg == "--verbose");

    // Check for help
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return Ok(());
    }

    // Check for configuration mode
    if args.iter().any(|arg| arg == "-c" || arg == "--config") {
        return create_config();
    }

    // Check for reload mode
    if args.iter().any(|arg| arg == "-r" || arg == "--reload") {
        return reload_config(verbose);
    }

    // Platform-specific behavior
    #[cfg(target_os = "windows")]
    {
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
            return run_tray_app(verbose);
        }

        // Check if running in console mode (for debugging)
        if args.iter().any(|arg| arg == "--console") {
            // Allocate a console for this GUI app so we can see output
            unsafe {
                use windows::Win32::System::Console::AllocConsole;
                let _ = AllocConsole();
            }
            return run_console_mode(verbose);
        }

        // Default behavior on Windows: run as tray app
        info!("Starting as tray application (default)");
        return run_tray_app(verbose);
    }

    // Non-Windows platforms use the original logic
    #[cfg(not(target_os = "windows"))]
    {
        let monitor = platforms::create_monitor(verbose)?;

        println!("QMK Window Notifier started");
        if verbose {
            println!("Verbose logging enabled");
            println!("Using platform: {}", monitor.platform_name());
        }

        // Set up signal handling for immediate exit
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down...");
            // Force immediate exit - no waiting or additional complexity
            process::exit(0);
        })?;

        // Start the monitor in a separate thread for non-Hyprland Linux and macOS
        #[cfg(any(
            all(target_os = "linux", not(feature = "hyprland")),
            target_os = "macos"
        ))]
        let monitor_thread = std::thread::spawn(move || {
            if let Err(e) = monitor.start() {
                eprintln!("Monitor error: {}", e);
            }
        });

        // Setup tray icon for all platforms except Hyprland
        #[cfg(not(all(target_os = "linux", feature = "hyprland")))]
        tray::setup_tray();

        #[cfg(not(all(target_os = "linux", feature = "hyprland")))]
        if verbose {
            println!("System tray icon initialized");
        }

        // Join the monitor thread for platforms where it was spawned
        #[cfg(any(
            all(target_os = "linux", not(feature = "hyprland")),
            target_os = "macos"
        ))]
        if let Err(e) = monitor_thread.join() {
            eprintln!("Error joining Monitor thread: {:?}", e);
        }

        // For Hyprland, start the monitor on the main thread
        #[cfg(all(target_os = "linux", feature = "hyprland"))]
        if let Err(e) = monitor.start() {
            eprintln!("Monitor error: {}", e);
        }

        // If we reach here, the monitor stopped on its own
        println!("Monitor stopped, exiting.");
    }

    #[cfg(not(target_os = "windows"))]
    return Ok(());
}

fn print_help() {
    println!("QMK Window Notifier v{}", env!("CARGO_PKG_VERSION"));
    println!("Usage: qmk-window-notifier [OPTIONS]");
    println!("\nOptions:");
    println!("  -h, --help     Display this help message");
    println!("  -v, --verbose  Enable verbose logging");
    println!("  -c, --config   Create a configuration file");
    println!("  -r, --reload   Reload configuration and update system files");
    println!("  -l, --list     List supported platforms");
    
    #[cfg(target_os = "windows")]
    {
        println!("\nWindows Options:");
        println!("  --console              Run in console mode (for debugging)");
        println!("  --tray-app             Run as tray application");
        println!("\nWindows Service Options:");
        println!("  --service              Run as Windows service (used internally)");
        println!("  --install-service      Install Windows service");
        println!("  --uninstall-service    Uninstall Windows service");
        println!("  --start-service        Start Windows service");
        println!("  --stop-service         Stop Windows service");
    }
    
    println!("\nRunning without options will start the notifier service");
}

#[cfg(target_os = "windows")]
fn is_already_running() -> Result<bool, Box<dyn Error>> {
    use single_instance::SingleInstance;
    
    // Create a single instance using a unique app ID
    // This will create a named mutex under the hood
    static mut INSTANCE: Option<SingleInstance> = None;
    
    let instance = SingleInstance::new("qmk-window-notifier-app-id").map_err(|e| -> Box<dyn Error> {
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

#[cfg(target_os = "windows")]
fn run_console_mode(verbose: bool) -> Result<(), Box<dyn Error>> {
    // This runs the original console-based logic for Windows debugging
    println!("Creating Windows monitor...");
    let monitor = platforms::create_monitor(verbose)?;

    println!("QMKonnect started in console mode");
    if verbose {
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

    if verbose {
        println!("Windows monitor started successfully");
    }

    // Keep the console app running
    println!("Press Ctrl+C to exit...");
    println!("Now switch between different applications to test window detection...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(target_os = "windows")]
fn run_tray_app(verbose: bool) -> Result<(), Box<dyn Error>> {
    // Check for existing instance (singleton)
    if is_already_running()? {
        if verbose {
            println!("Another instance is already running, exiting");
        }
        info!("Another instance is already running, exiting");
        return Ok(());
    }

    if verbose {
        println!("No other instance detected, starting application");
    }
    info!("Starting QMK Window Notifier as tray application");
    
    // Create the monitor
    let monitor = platforms::create_monitor(verbose)?;
    
    if verbose {
        info!("Using platform: {}", monitor.platform_name());
    }

    // On Windows, start the monitor before setting up the tray (like original working code)
    let mut monitor = monitor;
    if let Err(e) = monitor.start() {
        error!("Failed to start Windows monitor: {}", e);
        return Err(e);
    }
    if verbose {
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

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get platform-specific config paths
    let config_paths = platforms::get_config_paths();

    // Try each path in order
    for path in config_paths {
        if path.exists() {
            return Ok(path);
        }
    }

    Err("No configuration file found in any of the expected locations".into())
}

fn reload_config(verbose: bool) -> Result<(), Box<dyn Error>> {
    println!("Reloading configuration...");

    // Get config path
    let config_path = match get_config_path() {
        Ok(path) => path,
        Err(e) => {
            println!("Note: Could not update system configuration: {}", e);
            return Ok(());
        }
    };

    // Parse configuration using our improved parser
    let config = core::parse_config(&config_path)?;

    // The values are already u16
    let vendor_id = config.vendor_id;
    let product_id = config.product_id;

    if verbose {
        println!("Read configuration from {}", config_path.display());
        println!(
            "Using vendor_id: {:#06x}, product_id: {:#06x}",
            vendor_id, product_id
        );
    }

    // Update platform-specific configuration
    #[cfg(target_os = "linux")]
    {
        // Convert the numeric values back to hex strings for udev rules
        let vendor_id_hex = platforms::decimal_to_hex(vendor_id);
        let product_id_hex = platforms::decimal_to_hex(product_id);

        if let Err(e) = platforms::update_udev_rules(vendor_id_hex, product_id_hex, verbose) {
            if verbose {
                println!("Warning: Could not update udev rules: {}", e);
            }
        }

        if let Err(e) = platforms::reload_udev_rules() {
            if verbose {
                println!("Warning: Could not reload udev rules: {}", e);
            }
        }
    }

    println!("Configuration reloaded successfully.");
    Ok(())
}

fn create_config() -> Result<(), Box<dyn Error>> {
    println!("Creating configuration...");

    // Create config directory with platform-specific method
    #[cfg(target_os = "linux")]
    let config_dir = platforms::create_config_dir()?;

    #[cfg(target_os = "windows")]
    let config_dir = platforms::create_config_dir()?;

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    let config_dir = {
        // Default implementation for other platforms
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config).join("qmk-notifier")
        } else if let Some(home) = dirs::home_dir() {
            home.join(".config").join("qmk-notifier")
        } else {
            return Err("Could not determine configuration directory".into());
        }
    };

    // Create the config file using our new function
    let config_path = config_dir.join("config.toml");
    core::create_default_config(&config_path)?;

    Ok(())
}

#![cfg(target_os = "windows")]

use crate::platforms;
use crate::tray;
use log::{error, info};
use std::error::Error;
use std::ffi::OsString;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::{define_windows_service, service_dispatcher};

const SERVICE_NAME: &str = "QMKWindowNotifier";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub struct ServiceContext {
    shutdown_tx: Option<mpsc::Sender<()>>,
    monitor: Option<Box<dyn platforms::WindowMonitor>>,
}

impl ServiceContext {
    fn new() -> Self {
        Self {
            shutdown_tx: None,
            monitor: None,
        }
    }
}

// Global service context
static SERVICE_CONTEXT: Mutex<Option<ServiceContext>> = Mutex::new(None);

define_windows_service!(ffi_service_main, service_main);

pub fn run_service() -> Result<(), Box<dyn Error>> {
    info!("Starting QMK Window Notifier service dispatcher");
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

fn service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service_impl() {
        error!("Service error: {}", e);
    }
}

fn run_service_impl() -> Result<(), Box<dyn Error>> {
    info!("QMK Window Notifier service starting");

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    
    // Initialize service context
    {
        let mut context = SERVICE_CONTEXT.lock().unwrap();
        *context = Some(ServiceContext::new());
        if let Some(ref mut ctx) = *context {
            ctx.shutdown_tx = Some(shutdown_tx.clone());
        }
    }

    // Register service control handler
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                info!("Service stop/shutdown requested");
                if let Err(e) = shutdown_tx.send(()) {
                    error!("Failed to send shutdown signal: {}", e);
                }
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // Set service status to running
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    info!("Service status set to running");

    // Start the window monitor in a separate thread
    let monitor_handle = thread::spawn(move || {
        if let Err(e) = start_monitor() {
            error!("Monitor thread error: {}", e);
        }
    });

    // Start system tray in a separate thread
    let _tray_handle = thread::spawn(move || {
        info!("Starting system tray");
        tray::setup_tray();
    });

    // Wait for shutdown signal
    match shutdown_rx.recv() {
        Ok(()) => info!("Shutdown signal received"),
        Err(e) => error!("Error receiving shutdown signal: {}", e),
    }

    // Stop the monitor
    {
        let mut context = SERVICE_CONTEXT.lock().unwrap();
        if let Some(ref mut ctx) = *context {
            if let Some(ref mut monitor) = ctx.monitor {
                if let Err(e) = monitor.stop() {
                    error!("Error stopping monitor: {}", e);
                }
            }
        }
    }

    // Set service status to stopped
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    info!("Service stopped successfully");

    // Wait for threads to finish (with timeout)
    let _ = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        std::process::exit(0);
    });

    if let Err(e) = monitor_handle.join() {
        error!("Error joining monitor thread: {:?}", e);
    }

    // Note: tray_handle may not join cleanly due to event loop, that's expected

    Ok(())
}

fn start_monitor() -> Result<(), Box<dyn Error>> {
    info!("Creating window monitor");
    let mut monitor = platforms::create_monitor(false)?; // Service runs non-verbose by default
    
    // Store monitor in context for cleanup
    {
        let mut context = SERVICE_CONTEXT.lock().unwrap();
        if let Some(ref mut _ctx) = *context {
            // We can't store the monitor directly due to trait object limitations
            // The monitor will be managed by this thread
        }
    }

    info!("Starting window monitor");
    monitor.start()?;
    
    Ok(())
}

pub fn install_service() -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy();
    
    info!("Installing service from: {}", exe_path_str);
    
    let output = Command::new("sc")
        .args(&[
            "create",
            SERVICE_NAME,
            &format!("binPath=\"{}\" --service", exe_path_str),
            "start=auto",
            "DisplayName=QMK Window Notifier",
            "depend=Tcpip",
        ])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to install service: {}", error_msg).into());
    }

    info!("Service installed successfully");
    
    // Set service description
    let _ = Command::new("sc")
        .args(&[
            "description",
            SERVICE_NAME,
            "Monitors window focus changes and sends notifications to QMK keyboards",
        ])
        .output();

    Ok(())
}

pub fn uninstall_service() -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    
    info!("Uninstalling service");
    
    // Stop service first
    let _ = Command::new("sc")
        .args(&["stop", SERVICE_NAME])
        .output();

    // Delete service
    let output = Command::new("sc")
        .args(&["delete", SERVICE_NAME])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to uninstall service: {}", error_msg).into());
    }

    info!("Service uninstalled successfully");
    Ok(())
}

pub fn start_service() -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    
    info!("Starting service");
    
    let output = Command::new("sc")
        .args(&["start", SERVICE_NAME])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to start service: {}", error_msg).into());
    }

    info!("Service started successfully");
    Ok(())
}

pub fn stop_service() -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    
    info!("Stopping service");
    
    let output = Command::new("sc")
        .args(&["stop", SERVICE_NAME])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to stop service: {}", error_msg).into());
    }

    info!("Service stopped successfully");
    Ok(())
}
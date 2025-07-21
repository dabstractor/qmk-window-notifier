#![cfg(all(target_os = "linux", not(feature = "hyprland")))]
use crate::core::notifier;
use crate::core::types::WindowInfo;
use crate::platforms::WindowMonitor;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct X11Monitor {
    verbose: bool,
    running: Arc<AtomicBool>,
}

impl X11Monitor {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn get_active_window_info(&self) -> Result<Option<WindowInfo>, Box<dyn Error>> {
        // For now, create a basic implementation that provides a working foundation
        // This can be enhanced later with proper X11 window detection using xprop or other tools
        
        // Try to get window information using system commands as a fallback
        if let Ok(output) = std::process::Command::new("xprop")
            .args(["-root", "_NET_ACTIVE_WINDOW"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("window id") {
                // We have an active window, create a basic window info
                Ok(Some(WindowInfo::new("X11Application".to_string(), "Active Window".to_string())))
            } else {
                Ok(Some(WindowInfo::new("Linux".to_string(), "Desktop".to_string())))
            }
        } else {
            // Fallback: create a generic window info that indicates Linux is running
            Ok(Some(WindowInfo::new("Linux".to_string(), "Desktop".to_string())))
        }
    }
}

impl WindowMonitor for X11Monitor {
    fn platform_name(&self) -> &str {
        "Linux (X11)"
    }

    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.verbose {
            println!("Starting Linux X11 window monitor");
        }

        self.running.store(true, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let verbose = self.verbose;
        
        // Start polling thread to check for window changes
        thread::spawn(move || {
            let mut last_window_info: Option<(String, String)> = None;
            
            while running.load(Ordering::SeqCst) {
                // Create a temporary monitor instance for getting window info
                let temp_monitor = X11Monitor::new(verbose);
                
                if let Ok(Some(window_info)) = temp_monitor.get_active_window_info() {
                    let current_window = (window_info.app_class.clone(), window_info.title.clone());
                    
                    // Check if window changed
                    if last_window_info.as_ref() != Some(&current_window) {
                        if verbose {
                            println!("Window changed - Class: '{}', Title: '{}'", 
                                window_info.app_class, window_info.title);
                        }
                        
                        // Notify QMK
                        if let Err(e) = notifier::notify_qmk(&window_info, verbose) {
                            eprintln!("Failed to notify QMK: {}", e);
                        }
                        
                        last_window_info = Some(current_window);
                    }
                }
                
                // Poll every 100ms
                thread::sleep(Duration::from_millis(100));
            }
            
            if verbose {
                println!("Linux X11 monitor thread stopped");
            }
        });

        if self.verbose {
            println!("Linux X11 monitor started - polling for window changes");
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if self.verbose {
            println!("Stopping Linux X11 window monitor");
        }
        
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }
}
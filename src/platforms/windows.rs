#![cfg(target_os = "windows")]
use crate::core::notifier;
use crate::core::types::WindowInfo;
use crate::platforms::WindowMonitor;
use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    EVENT_OBJECT_FOCUS, WINEVENT_OUTOFCONTEXT,
};

static mut G_VERBOSE: bool = false;
static mut G_HOOK: Option<HWINEVENTHOOK> = None;
static mut LAST_WINDOW_INFO: Option<(String, String)> = None;

pub struct WindowsMonitor {
    verbose: bool,
    running: Arc<AtomicBool>,
}

impl WindowsMonitor {
    pub fn new(verbose: bool) -> Self {
        Self { 
            verbose,
            running: Arc::new(AtomicBool::new(false)),
        }
    }


}

impl WindowMonitor for WindowsMonitor {
    fn platform_name(&self) -> &str {
        "Windows"
    }

    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.verbose {
            println!("Starting Windows window monitor");
        }
        
        unsafe {
            G_VERBOSE = self.verbose;
            let _h_instance = GetModuleHandleA(None).unwrap_or_default();
            let hook = SetWinEventHook(
                EVENT_OBJECT_FOCUS,
                EVENT_OBJECT_FOCUS,
                None, // Use None for h_instance when using WINEVENT_OUTOFCONTEXT
                Some(event_proc),
                0,
                0,
                WINEVENT_OUTOFCONTEXT,
            );
            
            if hook.0 == 0 {
                return Err("Failed to set up Windows event hook".into());
            }
            
            G_HOOK = Some(hook);

            if self.verbose {
                println!("Windows event hook established successfully");
            }

            // Initial notification for the currently active window
            handle_focus_change(GetForegroundWindow());
        }
        
        self.running.store(true, Ordering::SeqCst);
        
        // Start a background thread to poll for window changes as a fallback
        let running = Arc::clone(&self.running);
        let verbose = self.verbose;
        thread::spawn(move || {
            if verbose {
                println!("Starting Windows polling thread as fallback");
            }
            
            let mut last_hwnd = unsafe { GetForegroundWindow() };
            
            while running.load(Ordering::SeqCst) {
                unsafe {
                    let current_hwnd = GetForegroundWindow();
                    if current_hwnd.0 != last_hwnd.0 && current_hwnd.0 != 0 {
                        if verbose {
                            println!("Polling detected window change (HWND: {:?})", current_hwnd.0);
                        }
                        handle_focus_change(current_hwnd);
                        last_hwnd = current_hwnd;
                    }
                }
                // Check every 100ms
                thread::sleep(Duration::from_millis(100));
            }
            
            if verbose {
                println!("Windows polling thread stopped");
            }
        });
        
        if self.verbose {
            println!("Windows monitor started - events will be processed automatically");
            println!("Try switching between different applications to test window detection");
        }
        
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if self.verbose {
            println!("Stopping Windows window monitor");
        }
        let hook = unsafe { ptr::replace(&raw mut G_HOOK, None) };
        if let Some(hook) = hook {
            unsafe {
                UnhookWinEvent(hook);
            }
        }
        Ok(())
    }
}

unsafe extern "system" fn event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    handle_focus_change(hwnd);
}

fn handle_focus_change(hwnd: HWND) {
    if let Ok(Some(window_info)) = get_window_info(hwnd) {
        // Filter out Windows internal components and empty windows
        if should_ignore_window(&window_info) {
            unsafe {
                if G_VERBOSE {
                    println!("Ignoring internal window - Class: '{}', Title: '{}'", 
                        window_info.app_class, window_info.title);
                }
            }
            return;
        }

        // Check if this is the same window as last time to prevent feedback loops
        let current_window = (window_info.app_class.clone(), window_info.title.clone());
        unsafe {
            if let Some(ref last_window) = LAST_WINDOW_INFO {
                if *last_window == current_window {
                    if G_VERBOSE {
                        println!("Duplicate window event ignored - Class: '{}', Title: '{}'", 
                            window_info.app_class, window_info.title);
                    }
                    return;
                }
            }
            LAST_WINDOW_INFO = Some(current_window);
        }

        unsafe {
            if G_VERBOSE {
                println!("Window focus changed - Class: '{}', Title: '{}'", 
                    window_info.app_class, window_info.title);
            }
        }
        
        if let Err(e) = notifier::notify_qmk(&window_info, unsafe { G_VERBOSE }) {
            eprintln!("Failed to notify QMK: {}", e);
        }
    }
}

fn should_ignore_window(window_info: &WindowInfo) -> bool {
    // Filter out Windows internal components
    let ignore_classes = [
        "ForegroundStaging",
        "XamlExplorerHostIslandWindow", 
        "Windows.UI.Composition.DesktopWindowContentBridge",
        "Windows.UI.Input.InputSite.WindowClass",
        "TaskSwitcherWnd",
        "TaskSwitcherOverlayWnd",
        "Windows.UI.Core.CoreWindow",
        "ApplicationFrameWindow", // UWP app frame (we want the actual content)
    ];

    // Ignore if class name matches internal components
    if ignore_classes.iter().any(|&class| window_info.app_class == class) {
        return true;
    }

    // Ignore windows with empty titles and certain class patterns
    if window_info.title.is_empty() {
        // Allow some specific classes even with empty titles (like some games or tools)
        let allow_empty_title = [
            "CASCADIA_HOSTING_WINDOW_CLASS", // Terminal apps
            "Chrome_WidgetWin_1", // Chrome/Electron apps
        ];
        
        if !allow_empty_title.iter().any(|&class| window_info.app_class == class) {
            return true;
        }
    }

    // Ignore very short titles that are likely not real applications
    if window_info.title.len() < 2 && !window_info.title.is_empty() {
        return true;
    }

    false
}

fn get_window_info(hwnd: HWND) -> Result<Option<WindowInfo>, Box<dyn Error>> {
    unsafe {
        if hwnd.0 == 0 {
            return Ok(None);
        }

        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id as *mut u32));

        let mut class_name_w: [u16; 256] = [0; 256];
        let class_name_len = GetClassNameW(hwnd, &mut class_name_w);
        let app_class = if class_name_len > 0 {
            let os_string = OsString::from_wide(&class_name_w[..class_name_len as usize]);
            os_string.to_string_lossy().into_owned()
        } else {
            String::new()
        };

        let mut window_text_w: [u16; 512] = [0; 512];
        let window_text_len = GetWindowTextW(hwnd, &mut window_text_w);
        let title = if window_text_len > 0 {
            let os_string = OsString::from_wide(&window_text_w[..window_text_len as usize]);
            os_string.to_string_lossy().into_owned()
        } else {
            String::new()
        };

        Ok(Some(WindowInfo::new(app_class, title)))
    }
}

// Windows-specific configuration path handling
pub fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Primary location: %APPDATA%\QMKonnect\config.toml
    if let Ok(app_data) = std::env::var("APPDATA") {
        paths.push(
            PathBuf::from(app_data)
                .join("QMKonnect")
                .join("config.toml"),
        );
    }

    // Secondary location: %LOCALAPPDATA%\QMKonnect\config.toml
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        paths.push(
            PathBuf::from(local_app_data)
                .join("QMKonnect")
                .join("config.toml"),
        );
    }

    // Fallback to executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("config.toml"));
        }
    }

    paths
}

// Create Windows configuration directory
pub fn create_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    // Use %APPDATA% for user configuration
    let config_dir = if let Ok(app_data) = std::env::var("APPDATA") {
        PathBuf::from(app_data).join("QMKonnect")
    } else {
        return Err("Could not determine APPDATA directory".into());
    };

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&config_dir)?;
    
    Ok(config_dir)
}
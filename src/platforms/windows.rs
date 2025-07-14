#![cfg(target_os = "windows")]
use crate::core::notifier;
use crate::core::types::WindowInfo;
use crate::platforms::WindowMonitor;
use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId, PostThreadMessageA,
    EVENT_OBJECT_FOCUS, WINEVENT_OUTOFCONTEXT, WM_QUIT,
};

static mut G_VERBOSE: bool = false;
static mut G_HOOK: Option<HWINEVENTHOOK> = None;
static G_THREAD_ID: AtomicU32 = AtomicU32::new(0);

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
        self.running.store(true, Ordering::SeqCst);
        unsafe {
            G_VERBOSE = self.verbose;
        }

        let running = self.running.clone();
        let handle = thread::spawn(move || {
            unsafe {
                G_THREAD_ID.store(thread::current().id().as_u32(), Ordering::SeqCst);
                let h_instance = GetModuleHandleA(None).unwrap_or_default();
                let hook = SetWinEventHook(
                    EVENT_OBJECT_FOCUS,
                    EVENT_OBJECT_FOCUS,
                    h_instance,
                    Some(event_proc),
                    0,
                    0,
                    WINEVENT_OUTOFCONTEXT,
                );
                G_HOOK = Some(hook);

                // Initial notification for the currently active window
                handle_focus_change();

                // Message loop
                let mut msg = std::mem::MaybeUninit::uninit();
                while running.load(Ordering::SeqCst) {
                    let b_ret = windows::Win32::UI::WindowsAndMessaging::GetMessageA(
                        msg.as_mut_ptr(),
                        HWND(0),
                        0,
                        0,
                    );
                    if b_ret == BOOL(0) || b_ret == BOOL(-1) {
                        break;
                    }
                    windows::Win32::UI::WindowsAndMessaging::TranslateMessage(msg.as_ptr());
                    windows::Win32::UI::WindowsAndMessaging::DispatchMessageA(msg.as_ptr());
                }

                if let Some(hook) = G_HOOK {
                    UnhookWinEvent(hook);
                }
            }
        });
        
        // We don't want to block the main thread, so we'll just let the hook run in the background.
        // The tray icon will keep the application alive.
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.running.store(false, Ordering::SeqCst);
        if let Some(hook) = unsafe { G_HOOK.take() } {
            unsafe {
                UnhookWinEvent(hook);
                let thread_id = G_THREAD_ID.load(Ordering::SeqCst);
                if thread_id != 0 {
                    PostThreadMessageA(thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
                }
            }
        }
        Ok(())
    }
}

extern "system" fn event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    handle_focus_change();
}

fn handle_focus_change() {
    if let Ok(Some(window_info)) = get_active_window_info() {
        if let Err(e) = notifier::notify_qmk(&window_info, unsafe { G_VERBOSE }) {
            eprintln!("Failed to notify QMK: {}", e);
        }
    }
}

fn get_active_window_info() -> Result<Option<WindowInfo>, Box<dyn Error>> {
    unsafe {
        let hwnd = GetForegroundWindow();
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

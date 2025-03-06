use crate::core::types::WindowInfo;
use once_cell::sync::Lazy;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Static state for debouncing
static DEBOUNCER: Lazy<Arc<Mutex<DebounceState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DebounceState {
        last_message: None,
        last_activity: Instant::now(),
        timer_running: false,
    }))
});

struct DebounceState {
    last_message: Option<String>,
    last_activity: Instant,
    timer_running: bool,
}

fn get_debouncer() -> Arc<Mutex<DebounceState>> {
    Arc::clone(&DEBOUNCER)
}

pub fn notify_qmk(window_info: &WindowInfo, verbose: bool) -> Result<(), Box<dyn Error>> {
    let message = format!("{}{}{}", window_info.app_class, "\x1D", window_info.title);
    let debouncer = get_debouncer();

    // Wrap in a block to limit the lock scope
    let should_send_now = {
        let mut state = debouncer.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_activity);

        // Update activity timestamp
        state.last_activity = now;

        // Store the latest message
        state.last_message = Some(message.clone());

        // Determine if we should send immediately
        let should_send_now = elapsed > Duration::from_millis(500) || state.last_message.is_none();

        // Start the timer thread if not already running
        if !state.timer_running && !should_send_now {
            state.timer_running = true;
            let debouncer_clone = Arc::clone(&debouncer);
            thread::spawn(move || debounce_timer(debouncer_clone));
        }

        should_send_now
    };

    // Send immediately if needed
    if should_send_now {
        if verbose {
            let sanitized_message = message.replace('\x1D', "|");
            println!("Notified QMK (immediate): {}", sanitized_message);
        }
        qmk_notifier::run(Some(message))?;
    } else if verbose {
        let sanitized_message = message.replace('\x1D', "|");
        println!("Debouncing notification: {}", sanitized_message);
    }

    Ok(())
}

fn debounce_timer(debouncer: Arc<Mutex<DebounceState>>) {
    loop {
        thread::sleep(Duration::from_millis(100)); // Check every 100ms

        let (should_exit, message_to_send, verbose_mode) = {
            let mut state = debouncer.lock().unwrap();
            let now = Instant::now();
            let elapsed = now.duration_since(state.last_activity);

            // If 1 second has passed without activity, send the last message
            if elapsed >= Duration::from_millis(500) {
                let message = state.last_message.clone();
                state.last_message = None;
                state.timer_running = false;
                (true, message, false) // Exit after sending
            } else {
                (false, None, false) // Continue waiting
            }
        };

        // Send the debounced message if needed
        if let Some(message) = message_to_send {
            if verbose_mode {
                let sanitized_message = message.replace('\x1D', "|");
                println!("Notified QMK (debounced): {}", sanitized_message);
            }
            let _ = qmk_notifier::run(Some(message));
        }

        if should_exit {
            break;
        }
    }
}

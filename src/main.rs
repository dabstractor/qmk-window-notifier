use hyprland::{
    data::Client,
    event_listener::{EventListener, WindowEventData},
    shared::HyprData,
    shared::HyprDataActiveOptional,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Check if HYPRLAND_INSTANCE_SIGNATURE is set
    match std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(signature) => {
            if is_verbose() {
                println!("Found Hyprland signature: {}", signature)
            }
        }
        Err(_) => {
            eprintln!("HYPRLAND_INSTANCE_SIGNATURE environment variable not set!");
            eprintln!("Are you running this program within Hyprland?");
            return Err("Missing HYPRLAND_INSTANCE_SIGNATURE".into());
        }
    }

    // Create a new event listener and set up handlers
    let mut event_listener = EventListener::new();
    event_listener.add_active_window_changed_handler(|window_event| {
        if let Err(err) = handle_active_window_change(window_event) {
            eprintln!("Error handling active window change: {}", err);
        }
    });

    // Try to verify connection before setting up handlers
    if let Err(e) = hyprland::data::Monitors::get() {
        eprintln!("Failed to connect to Hyprland: {}", e);
        return Err("Could not establish initial connection to Hyprland".into());
    }

    // Start the listener with error handling
    match event_listener.start_listener() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to start event listener: {}", e);
            Err(e.into())
        }
    }
}

fn handle_active_window_change(
    _window_event: Option<WindowEventData>,
) -> Result<(), Box<dyn Error>> {
    // Using Client::get_active() which returns Result<Option<Client>, HyprError>
    match Client::get_active() {
        Ok(Some(active_window)) => {
            // Get the initialClass property
            let initial_class = &active_window.initial_class;
            let title = &active_window.title;

            notify_qmk(&format!("{}{}{}", initial_class, "\x1D", title))?;
        }
        Ok(None) => {
            println!("No active window found");
        }
        Err(err) => {
            println!("Failed to get active window info: {}", err);
        }
    }

    Ok(())
}

fn notify_qmk(window_class: &str) -> Result<(), Box<dyn Error>> {
    // Call the run function from the qmk_notifier package
    // and I don't want it to cause this program to exit
    //
    let _ = qmk_notifier::run(Some(window_class.to_string()));

    if is_verbose() {
        let sanitized_window_class = window_class.replace('\x1D', "|");
        println!("Notified QMK of window class: {}", sanitized_window_class);
    }

    Ok(())
}

fn is_verbose() -> bool {
    return std::env::args().any(|arg| arg == "-v");
}

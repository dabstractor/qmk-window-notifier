#![cfg(not(all(target_os = "linux", feature = "hyprland")))]
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

#[cfg(target_os = "macos")]
use std::env;



enum UserEvent {
    MenuEvent(MenuEvent),
}

pub fn setup_tray() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let tray_menu = Menu::new();

    let quit_i = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("QMK Window Notifier".to_string()),
                copyright: Some("Copyright Mulletware 2025".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    let mut tray_icon = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                let icon = {
                    #[cfg(target_os = "macos")]
                    {
                        load_icon_from_bundle().unwrap_or_else(|_| create_default_icon())
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        // Try to find icon relative to executable first
                        let exe_path = std::env::current_exe().unwrap_or_default();
                        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
                        let icon_path = exe_dir.join("Icon.png");
                        
                        if icon_path.exists() {
                            load_icon(&icon_path).unwrap_or_else(|_| create_default_icon())
                        } else {
                            // Fallback to development path, then default icon
                            load_icon(std::path::Path::new("packaging/Icon.png"))
                                .unwrap_or_else(|_| create_default_icon())
                        }
                    }
                };

                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_tooltip("QMK Window Notifier")
                        .with_icon(icon)
                        .build()
                        .unwrap(),
                );



                // We have to request a redraw here to have the icon actually show up.
                // Tao only exposes a redraw method on the Window so we use core-foundation directly.
                #[cfg(target_os = "macos")]
                unsafe {
                    use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                    let rl = CFRunLoopGetMain().unwrap();
                    CFRunLoopWakeUp(&rl);
                }
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                println!("Exited");
                if event.id == quit_i.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}

fn load_icon(path: &std::path::Path) -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}

fn create_default_icon() -> tray_icon::Icon {
    // Create a simple 16x16 default icon if no icon file is found
    let rgba = vec![255u8; 16 * 16 * 4]; // White 16x16 icon
    tray_icon::Icon::from_rgba(rgba, 16, 16).expect("Failed to create default icon")
}


#[cfg(target_os = "macos")]
fn load_icon_from_bundle() -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let bundle = core_foundation::bundle::CFBundle::main_bundle();
    let bundle_path = bundle.executable_url().unwrap().to_path().unwrap();
    let resources_path = bundle_path.parent().unwrap().join("../Resources");
    let icon_path = resources_path.join("Icon.png");

    load_icon(&icon_path)
}
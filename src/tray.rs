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
                        load_icon_from_bundle()
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        load_icon(std::path::Path::new("../../packaging/Icon.png"))
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

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

#[cfg(target_os = "macos")]
fn load_icon_from_bundle() -> tray_icon::Icon {
    use objc2_foundation::{NSBundle, NSString};
    use std::path::Path;

    // Try multiple strategies to find the icon

    // 1. First, try to load from the bundle (for the built app)
    let bundle = NSBundle::mainBundle();
    let name = NSString::from_str("Icon");
    let ext = NSString::from_str("png");

    // Attempt to get the resource path from the bundle
    let bundle_resource = unsafe { bundle.pathForResource_ofType(Some(&name), Some(&ext)) };

    // If we successfully found the icon in the bundle, use it

    if let Some(resource) = bundle_resource {
        let resource_str = resource.to_string();
        return load_icon(Path::new(&resource_str));
    }

    // 2. Try to find the icon relative to the executable path
    if let Ok(exe_path) = env::current_exe() {
        // For app bundle: /path/to/App.app/Contents/MacOS/binary
        // Go up to Contents and check Resources
        let mut resources_path = exe_path.clone();
        // Go up to MacOS directory
        if resources_path.pop() {
            // Go up to Contents directory
            if resources_path.pop() {
                // Go to Resources directory
                resources_path.push("Resources");
                resources_path.push("Icon.png");

                if resources_path.exists() {
                    return load_icon(&resources_path);
                }
            }
        }

        // 3. Try relative to the executable (for cargo run/build)
        let relative_paths = vec![
            // Directly in the same directory
            exe_path.with_file_name("Icon.png"),
            // In packaging directory at project root
            {
                let mut p = exe_path.clone();
                // Go up to the binary directory
                p.pop();
                // For debug/release builds in target directory, go up multiple levels
                if p.ends_with("debug") || p.ends_with("release") {
                    p.pop(); // Up from debug/release
                    p.pop(); // Up from target
                    p.push("packaging");
                    p.push("Icon.png");
                }
                p
            },
        ];

        for path in relative_paths {
            if path.exists() {
                return load_icon(&path);
            }
        }
    }

    // 4. Try common development paths
    let dev_paths = [
        "packaging/Icon.png",
        "../packaging/Icon.png",
        "../../packaging/Icon.png",
        "../../../packaging/Icon.png",
        "../../../../packaging/Icon.png",
    ];

    for path in dev_paths {
        let path = Path::new(path);
        if path.exists() {
            return load_icon(path);
        }
    }

    // 5. Last resort: try to find it relative to the current working directory
    if let Ok(cwd) = env::current_dir() {
        let mut search_dir = cwd.clone();

        // Try to find a packaging directory by walking up the directory tree
        for _ in 0..5 {
            // Limit the search depth
            let icon_path = search_dir.join("packaging").join("Icon.png");
            if icon_path.exists() {
                return load_icon(&icon_path);
            }

            // Go up one directory
            if !search_dir.pop() {
                break;
            }
        }
    }

    // If all else fails, panic with a helpful error message
    panic!("Could not find Icon.png in any expected location. Please ensure the icon is placed in the packaging directory or in the app bundle's Resources directory.");
}

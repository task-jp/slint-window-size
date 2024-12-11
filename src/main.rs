// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let size = ui.get_window_size();
    println!("Initial Window Size: {}x{}", size.width, size.height);

    let ui_handle = ui.as_weak().unwrap();
    ui.on_window_size_changed(move || {
        let size = ui_handle.get_window_size();
        println!("Window Size Changed: {}x{}", size.width, size.height);
    });

    ui.run()?;

    Ok(())
}

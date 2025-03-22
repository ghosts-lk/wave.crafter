mod synthesizer;
mod ui;
mod mixer; // Updated to declare directly
mod audio; // Updated to declare directly

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Spinner, Box, Orientation};
use glib::MainContext;

use std::thread;
use std::time::Duration;

fn initialize_app() {
    // Simulate heavy initialization tasks
    thread::sleep(Duration::from_secs(5)); // Replace with actual initialization logic
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.wavecrafter")
        .build();

    app.connect_activate(|app| {
        // Create the splash screen
        let splash = ApplicationWindow::builder()
            .application(app)
            .title("Loading...")
            .default_width(300)
            .default_height(200)
            .build();

        let spinner = Spinner::builder().build();
        spinner.start();

        let label = Label::builder()
            .label("Wave Crafter is loading...")
            .margin_top(20)
            .build();

        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();
        vbox.pack_start(&spinner, false, false, 0);
        vbox.pack_start(&label, false, false, 0);

        splash.set_child(Some(&vbox));
        splash.show();

        // Run initialization in a separate task on the main thread
        let app_clone = app.clone();
        let splash_clone = splash.clone();
        MainContext::default().spawn_local(async move {
            initialize_app();

            // Once initialization is complete, show the main window
            splash_clone.close();

            let main_window = ApplicationWindow::builder()
                .application(&app_clone)
                .title("Wave Crafter")
                .default_width(800)
                .default_height(600)
                .build();

            main_window.show();
        });
    });

    app.run();
}

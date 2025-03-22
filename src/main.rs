mod synthesizer; // Import synthesizer module
mod ui;          // Import UI module
mod mixer;       // Import mixer module
mod audio;       // Import audio module

use gtk::prelude::*; // Import GTK traits for UI components
use gtk::{Application, ApplicationWindow, Label, Spinner, Box as GtkBox, Orientation}; // Rename GTK Box to GtkBox
use glib::MainContext; // Import glib::MainContext for async operations

use std::time::Duration;

/// Simulates application initialization tasks, such as loading resources or setting up dependencies.
async fn initialize_app() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate a delay for initialization
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
}

fn create_splash_screen(app: &Application) -> ApplicationWindow {
    let splash = ApplicationWindow::builder()
        .application(app)
        .title("Loading...") // Set splash screen title
        .default_width(300)
        .default_height(200)
        .decorated(false) // Make the window borderless
        .build();

    let spinner = Spinner::builder().build(); // Create a spinner for loading animation
    spinner.start(); // Start the spinner animation

    let label = Label::builder()
        .label("Loading...") // Set loading message
        .margin_top(20)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical) // Arrange components vertically
        .spacing(10)
        .build();
    vbox.pack_start(&spinner, false, false, 0); // Add spinner to the box
    vbox.pack_start(&label, false, false, 0);   // Add label to the box

    splash.set_child(Some(&vbox)); // Set the box as the child of the splash window
    splash
}

fn create_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Wave Crafter") // Set main window title
        .default_width(800)
        .default_height(600)
        .build()
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.wavecrafter") // Set application ID
        .build();

    app.connect_activate(|app| {
        let splash = create_splash_screen(app);
        splash.show(); // Display the splash screen

        let app_clone = app.clone();
        let splash_clone = splash.clone();
        MainContext::default().spawn_local(async move {
            if let Err(e) = initialize_app().await {
                eprintln!("Initialization error: {}", e); // Log initialization errors
                splash_clone.close();
                return;
            }

            splash_clone.close(); // Close the splash screen
            let main_window = create_main_window(&app_clone);
            main_window.show(); // Display the main application window
        });
    });

    app.run(); // Run the GTK application
}

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tracing::Level;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn build() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simulation",
        native_options,
        Box::new(|cc| Box::new(physics::State::new(cc))),
    )
    .unwrap()
}

fn main() {
    puffin::set_scopes_on(true);
    build();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn build() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(physics::State::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

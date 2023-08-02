#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use egui_node_graph_example::NodeGraphExample;

mod webserver;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;
    use tokio::runtime::Runtime;

    use crate::webserver::webserver;

    // Create a message channel for the web server thread
    let (server_tx, server_rx) = std::sync::mpsc::channel();

    // Start a web server thread
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        // Use the runtime
        rt.block_on(async {
            webserver(server_rx).await.unwrap();
        })
    });

    eframe::run_native(
        "Egui node graph example",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            #[cfg(feature = "persistence")]
            {
                Box::new(NodeGraphExample::new(cc))
            }
            #[cfg(not(feature = "persistence"))]
            Box::<NodeGraphExample>::default()
        }),
    )
    .expect("Failed to run native example");
}

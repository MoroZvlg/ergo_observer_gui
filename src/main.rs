#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod data;
mod state;
mod theme;
mod ui;
mod widgets;

use app::FundingApp;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

fn main() {
  #[cfg(target_arch = "wasm32")]
  {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logger");

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
      let document = web_sys::window().expect("No window").document().expect("No document");

      let canvas = document
        .get_element_by_id("the_canvas_id")
        .expect("Failed to find canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Element is not a canvas");

      eframe::WebRunner::new()
        .start(canvas, web_options, Box::new(move |cc| Ok(Box::new(FundingApp::new(cc)))))
        .await
        .expect("failed to start eframe");
    });
  }

  #[cfg(not(target_arch = "wasm32"))]
  {
    rustls::crypto::ring::default_provider()
      .install_default()
      .expect("Failed to install default crypto");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let native_options = eframe::NativeOptions {
      viewport: eframe::egui::ViewportBuilder::default()
        // .with_maximized(true),
        .with_inner_size([1200.0, 861.0]),
      vsync: false,
      ..Default::default()
    };

    eframe::run_native(
      "Funding Rates",
      native_options,
      Box::new(move |cc| Ok(Box::new(FundingApp::new(cc)))),
    )
    .expect("failed to run native app");
  }
}

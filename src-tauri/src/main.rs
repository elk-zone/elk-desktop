#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod system_tray;
mod utils;
mod window_state;

use crate::system_tray::{create_tray, system_tray_event_handler};
use env_logger::filter::Builder as FilterBuilder;
use log::LevelFilter;
use tauri::Menu;
#[cfg(debug_assertions)]
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Builder as LogPluginBuilder, LogTarget};
use tauri_plugin_store::Builder as StorePluginBuilder;
use window_state::Builder as WindowStateBuilder;

fn main() {
    let log_plugin = {
        let targets = [
            LogTarget::LogDir,
            #[cfg(debug_assertions)]
            LogTarget::Stdout,
        ];

        let filter = std::env::var("RUST_LOG")
            .map(|ref filter| FilterBuilder::new().parse(filter).build().filter())
            .unwrap_or(LevelFilter::Debug);

        let builder = LogPluginBuilder::new().targets(targets).level(filter);

        #[cfg(debug_assertions)]
        let builder = builder.with_colors(ColoredLevelConfig::default());

        builder.build()
    };

    let system_tray = create_tray();

    tauri::Builder::default()
        .plugin(log_plugin)
        .plugin(StorePluginBuilder::default().build())
        .plugin(WindowStateBuilder::default().build())
        .menu(Menu::os_default("Elk"))
        .system_tray(system_tray)
        .on_system_tray_event(system_tray_event_handler)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}

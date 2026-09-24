#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod afc;
mod airlift;
mod airtraffic;
mod app;
mod apple;
mod device;
mod flasher;
mod i18n;
mod image_skin;
mod native_stream;
mod native_sync;
mod pairing;
mod passthm;
mod scanner;
mod service_protocol;
mod usbmux;
mod wallet_backup;
mod wallet_connection;

fn main() -> eframe::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("--native-sync-worker") {
        std::process::exit(airtraffic::run_native_worker());
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 620.0])
            .with_min_inner_size([850.0, 560.0])
            .with_title(concat!("AirCard v", env!("CARGO_PKG_VERSION"))),
        ..Default::default()
    };

    eframe::run_native(
        concat!("AirCard v", env!("CARGO_PKG_VERSION")),
        options,
        Box::new(|cc| Ok(Box::new(app::AirCardApp::new(cc)))),
    )
}

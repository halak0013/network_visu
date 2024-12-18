pub mod main_ui;

use main_ui::NodeApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Kablosuz Ağ Simülasyonu",
        options,
        Box::new(|_cc| Ok(Box::<NodeApp>::default())),
    )
}

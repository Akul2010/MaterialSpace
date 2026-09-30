mod app;
mod database;
mod material;
mod physics;
mod ui;

use app::MaterialSpaceApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MaterialSpace",
        options,
        Box::new(|cc| {
            Ok(Box::new(
                MaterialSpaceApp::new(cc),
            ))
        }),
    )
}
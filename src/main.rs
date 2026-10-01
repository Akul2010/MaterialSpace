mod app;
mod database;
mod material;
mod metrics;
mod physics;
mod ui;

use app::MaterialSpaceApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 840.0])
            .with_min_inner_size([960.0, 640.0])
            .with_title("MaterialSpace — Engineering Materials Intelligence"),
        ..Default::default()
    };

    eframe::run_native(
        "MaterialSpace",
        options,
        Box::new(|cc| Ok(Box::new(MaterialSpaceApp::new(cc)))),
    )
}
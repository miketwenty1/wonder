use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example)
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    let code =
        QrCode::with_version(b"https://mydiscordlink.xyz", Version::Micro(2), EcLevel::L).unwrap();

    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();

    let a = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "testingrenderedsvg",
        image.as_bytes(),
        egui_extras::image::FitTo::Original,
    )
    .unwrap();

    egui::Window::new("qrcode").show(egui_context.ctx_mut(), |ui| {
        let max_size = ui.available_size();
        a.show_size(ui, max_size);
    });
}

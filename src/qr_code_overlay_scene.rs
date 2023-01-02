use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui_extras::RetainedImage;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};

/// QR Code to be rendered
#[derive(Resource, Deref)]
pub struct MyQr(pub RetainedImage);

#[derive(Resource, Deref, Default)]
pub struct CurrentQrString(pub String);

//
// pub struct ClippyBoy;

pub fn setup_qr(mut commands: Commands, qrcode_str: ResMut<CurrentQrString>) {
    //let qrdata = "LNBC20N1P36A7LPPP5UC2ZSMMZQU6ZEH8XW54MY6997EU8XYYKM25Y0EPLC09H9M53KC9QDQVTFZ5Y32YG4ZSCQZPGXQZJCSP5VH36CEYL3CRHC7KF0WWMVXAJJ2S4J50NV40DZV7VUMKKGYRZU80Q9QYYSSQW3380SLJFCJ2RSEHPLTAR57VRUYWN8YPJJQFTU8DC56NC36ST3USKHGVJJQJLKHN7AKKDPR8KALH74SLETQDUT8R02EH96LTPG4Y5LCQDFKTJ7";

    //qrcode_str.0 = qrdata.to_string();
    info!("{}", qrcode_str.0);
    let code = QrCode::with_version(&qrcode_str.0, Version::Normal(9), EcLevel::L).unwrap();
    //
    let image = code
        .render::<svg::Color>()
        .min_dimensions(200, 200)
        .max_dimensions(300, 300)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();

    let a = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "invoicesvg",
        image.as_bytes(),
        egui_extras::image::FitTo::Original,
    )
    .unwrap();

    // Cache QR code to be used later
    commands.insert_resource(MyQr(a))
}

#[cfg(web_sys_unstable_apis)]
pub fn update_qr(
    qr: Res<MyQr>,
    mut egui_context: ResMut<EguiContext>,
    qrcode_str: ResMut<CurrentQrString>,
) {
    egui::Window::new("BTC Invoice").show(egui_context.ctx_mut(), |ui| {
        // Size to smallest square to preserve dimensions
        let bevy_egui::egui::Vec2 { x, y } = ui.available_size();
        let smaller = x.min(y);
        qr.0.show_size(ui, bevy_egui::egui::Vec2::new(smaller, smaller));

        let button_min_size = bevy_egui::egui::Vec2 { x: 120.0, y: 60.0 };
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new("Copy to Clipboard").min_size(button_min_size))
                .clicked()
            {
                let qrcode_str = qrcode_str.clone();
                use wasm_bindgen_futures::spawn_local;
                spawn_local(async move {
                    let window = web_sys::window().expect("window"); // { obj: val };
                    let nav = window.navigator().clipboard();
                    match nav {
                        Some(a) => {
                            let p = a.write_text(&qrcode_str);
                            let _result = wasm_bindgen_futures::JsFuture::from(p)
                                .await
                                .expect("clipboard populated");
                            info!("clippyboy worked");
                        }
                        None => {
                            warn!("failed to copy clippyboy");
                        }
                    };
                });
                //info!("{}", qrcode_str.to_string());
            };
        });
    });
}

pub fn clean_qr(// qr: Res<MyQr>,
    // mut egui_context: ResMut<EguiContext>,
    // mut qrcode_str: ResMut<CurrentQrCode>,
) {
    // egui::Window::new("qrcode").show(egui_context.ctx_mut(), |ui| {
    //     // Size to smallest square to preserve dimensions
    //     let bevy_egui::egui::Vec2 { x, y } = ui.available_size();
    //     let smaller = x.min(y);
    //     qr.0.show_size(ui, bevy_egui::egui::Vec2::new(smaller, smaller));
    // });
}

// pub fn cleanup_player_scene(
//     //mut commands: Commands,
//     //mut qr: Res<MyQr>,
//     mut egui_context: ResMut<EguiContext>,
//     // mut keyboard_keys: Query<(Entity, With<EguiContext>)>,
// ) {
//     egui_context.ctx_mut().end_frame();
// }

// pub fn setup_qr_off() {}
// pub fn update_qr_off() {}
// pub fn clean_qr_off(
//     //mut commands: Commands,
//     // mut qr: Res<MyQr>,
//     mut egui_context: ResMut<EguiContext>,
//     // mut keyboard_keys: Query<(Entity, With<EguiContext>)>,
// ) {
//     egui_context.ctx_mut().end_frame();
// }

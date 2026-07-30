#![windows_subsystem = "windows"]

mod app;
mod config;
mod extract;

use app::XaApp;
use eframe::egui;

fn load_chinese_font(fonts: &mut egui::FontDefinitions) {
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "C:\\Windows\\Fonts\\msjh.ttc",
        "C:\\Windows\\Fonts\\yugoth.ttc",
    ];

    for font_path in &font_paths {
        if let Ok(data) = std::fs::read(font_path) {
            let font_name = format!("chinese_{}", font_path);
            fonts.font_data.insert(
                font_name.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(data)),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.clone());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font_name);
            return;
        }
    }
}

fn load_icon() -> Option<std::sync::Arc<egui::IconData>> {
    let icon_bytes = include_bytes!("../icon.jpg");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        return Some(std::sync::Arc::new(egui::IconData {
            rgba: rgba.into_raw(),
            width: w,
            height: h,
        }));
    }
    None
}

fn main() -> Result<(), eframe::Error> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([520.0, 500.0])
        .with_min_inner_size([420.0, 380.0])
        .with_resizable(true)
        .with_decorations(false);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Xa缓存加载工具",
        options,
        Box::new(|cc| {
            let mut font_defs = egui::FontDefinitions::default();
            load_chinese_font(&mut font_defs);
            cc.egui_ctx.set_fonts(font_defs);
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(XaApp::new()))
        }),
    )
}

use std::io::Write;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }

    // Convert icon.jpg to icon.ico (PNG-embedded ICO format)
    let icon_jpg = std::path::Path::new("icon.jpg");
    let icon_ico = std::path::Path::new("icon.ico");

    if icon_jpg.exists() && !icon_ico.exists() {
        if let Ok(img) = image::open(icon_jpg) {
            let (w, h) = (img.width(), img.height());
            // Save as PNG via the image crate
            let mut png_buf = std::io::Cursor::new(Vec::new());
            if img.write_to(&mut png_buf, image::ImageFormat::Png).is_ok() {
                let png_bytes = png_buf.into_inner();
                if let Ok(mut ico_file) = std::fs::File::create(icon_ico) {
                    // ICO header
                    ico_file.write_all(&[0, 0, 1, 0, 1, 0]).ok(); // reserved + type=icon + count=1
                    // ICO entry
                    let wb = w.min(256) as u8;
                    let hb = h.min(256) as u8;
                    let size = png_bytes.len() as u32;
                    let offset: u32 = 6 + 16;
                    ico_file.write_all(&[wb, hb, 0, 0, 1, 0, 32, 0]).ok();
                    ico_file.write_all(&size.to_le_bytes()).ok();
                    ico_file.write_all(&offset.to_le_bytes()).ok();
                    // PNG payload
                    ico_file.write_all(&png_bytes).ok();
                }
            }
        }
    }

    // Windows resource (version info + copyright)
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "XaCacheLoader");
    res.set("FileDescription", "Developed using rust and egui");
    res.set("LegalCopyright", "Copyright \u{00A9} 2026 Demon");
    res.set("OriginalFilename", "xa cache loader.exe");
    res.set("InternalName", "xa cache loader");
    res.set("ProductVersion", "1.0.0");
    res.set("FileVersion", "1.0.0");

    if icon_ico.exists() {
        res.set_icon(icon_ico.to_str().unwrap());
    }

    if let Err(e) = res.compile() {
        eprintln!("Failed to set Windows resource: {}", e);
    }
}

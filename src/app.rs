use chrono::Local;
use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;

use crate::config::Config;
use crate::extract;

#[derive(PartialEq)]
enum LoadState {
    Idle,
    Loading,
    Done,
}

enum BgMessage {
    Log(String),
    Done { bot_core_exists: bool },
}

pub struct XaApp {
    config: Config,
    exe_dir: PathBuf,
    target_path: String,
    logs: Vec<String>,
    state: LoadState,
    bot_core_exists: bool,
    bg_rx: Option<mpsc::Receiver<BgMessage>>,
}

impl XaApp {
    pub fn new() -> Self {
        let exe_dir = extract::get_exe_dir();
        let config = Config::load(&exe_dir);
        let display_path = config.custom_path.clone()
            .unwrap_or_else(|| exe_dir.to_string_lossy().to_string());
        let bot_core_exists = extract::check_bot_core(&config.get_target_path(&exe_dir));

        let mut app = Self {
            config,
            exe_dir,
            target_path: display_path,
            logs: Vec::new(),
            state: LoadState::Idle,
            bot_core_exists,
            bg_rx: None,
        };
        app.add_log("使用说明:");
        app.add_log("方法1: 可将APP和Xa缓存.zip放置在HanBot目录使用");
        app.add_log("方法2: 在任意目录运行APP, 同目录放置Xa缓存.zip, 手动设置目录点加载");
        app.add_log("提示: 如果没有放置Xa缓存.zip, 将加载内置的Xa缓存");
        app
    }

    fn add_log(&mut self, msg: &str) {
        let now = Local::now().format("%H:%M:%S").to_string();
        self.logs.push(format!("[{}] {}", now, msg));
    }

    fn do_load(&mut self) {
        let target_dir = self.config.get_target_path(&self.exe_dir);

        if !extract::check_bot_core(&target_dir) {
            self.add_log("未检测到BOT核心，加载已取消");
            self.bot_core_exists = false;
            self.state = LoadState::Idle;
            return;
        }

        let (tx, rx) = mpsc::channel();
        self.bg_rx = Some(rx);
        self.state = LoadState::Loading;

        let exe_dir = self.exe_dir.clone();
        let target_dir_clone = target_dir.clone();
        let embedded_7z = include_bytes!("../assets/hc.7z").to_vec();

        std::thread::spawn(move || {
            let log = |msg: &str| {
                let now = Local::now().format("%H:%M:%S").to_string();
                tx.send(BgMessage::Log(format!("[{}] {}", now, msg))).ok();
            };

            match extract::clear_saves_dir(&target_dir_clone) {
                Ok(_) => log("已清理 saves 目录"),
                Err(e) => log(&format!("清理saves目录失败: {}", e)),
            }

            let xa_zip = exe_dir.join("Xa缓存.zip");

            if xa_zip.exists() {
                log("检测到同目录下的Xa缓存.zip文件, 开始加载 Xa缓存");

                match extract::extract_zip(&xa_zip, &target_dir_clone, |_| {}) {
                    Ok(_) => {
                        let shard_src = target_dir_clone.join("Xalyn - Utils.shard");
                        let shards_dir = target_dir_clone.join("shards");
                        if shard_src.exists() {
                            let _ = extract::move_file_to_dir(&shard_src, &shards_dir);
                        }
                        let png_path = target_dir_clone.join("使用说明.png");
                        if png_path.exists() { let _ = extract::delete_file(&png_path); }
                        let txt_path = target_dir_clone.join("双合集使用及使用说明.txt");
                        if txt_path.exists() {
                            if let Ok(content) = std::fs::read_to_string(&txt_path) {
                                log(&format!("使用说明: {}", content));
                            }
                            let _ = std::fs::remove_file(&txt_path);
                        }

                        log("Xa缓存加载完成");
                    }
                    Err(e) => {
                        log(&format!("加载失败: {}", e));
                    }
                }
            } else {
                log("未检测到同目录下的Xa缓存.zip文件, 开始加载内置 Xa缓存");

                let temp_7z = exe_dir.join("_hc_temp.7z");
                match std::fs::write(&temp_7z, &embedded_7z) {
                    Ok(_) => {
                        match extract::extract_7z(&temp_7z, &target_dir_clone, |_| {}) {
                            Ok(_) => {
                                let _ = std::fs::remove_file(&temp_7z);
                                log("Xa缓存加载完成");
                            }
                            Err(e) => {
                                let _ = std::fs::remove_file(&temp_7z);
                                log(&format!("加载失败: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        log(&format!("无法解压内置缓存: {}", e));
                    }
                }
            }

            let bot_exists = extract::check_bot_core(&target_dir_clone);
            tx.send(BgMessage::Done {
                bot_core_exists: bot_exists,
            }).ok();
        });
    }

    fn browse_folder(&mut self) {
        let starting = if self.target_path.is_empty() {
            dirs::desktop_dir()
                .unwrap_or_else(|| PathBuf::from("C:\\"))
                .to_string_lossy()
                .to_string()
        } else {
            self.target_path.clone()
        };

        if let Some(path) = rfd::FileDialog::new().set_directory(starting).pick_folder() {
            let path_str = path.to_string_lossy().to_string();
            self.target_path = path_str.clone();
            self.config.custom_path = Some(path_str);
            self.config.save(&self.exe_dir);
            self.bot_core_exists =
                extract::check_bot_core(&self.config.get_target_path(&self.exe_dir));
        }
    }
}

impl eframe::App for XaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut done = false;
        let mut done_bot = false;
        if let Some(rx) = &self.bg_rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    BgMessage::Log(s) => self.logs.push(s),
                    BgMessage::Done { bot_core_exists } => {
                        done = true;
                        done_bot = bot_core_exists;
                    }
                }
            }
            if done {
                self.state = LoadState::Done;
                self.bot_core_exists = done_bot;
                self.bg_rx = None;
                self.config.save(&self.exe_dir);
            }
            if self.state == LoadState::Loading {
                ctx.request_repaint();
            }
        }

        // --- Top panel: title bar ---
        egui::TopBottomPanel::top("title_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                let title_rect = ui.allocate_space(
                    egui::vec2(ui.available_width() - 32.0, 20.0),
                ).1;
                let title_resp = ui.interact(title_rect, ui.next_auto_id(), egui::Sense::drag());
                if title_resp.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                ui.painter().text(
                    title_rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Xa 缓存加载工具",
                    egui::FontId::proportional(16.0),
                    ui.style().visuals.text_color(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_rect = ui.allocate_space(egui::vec2(24.0, 20.0)).1;
                    let close_resp = ui.interact(close_rect, ui.next_auto_id(), egui::Sense::click());
                    if close_resp.hovered() {
                        ui.painter().rect_filled(
                            close_rect, 2.0,
                            egui::Color32::from_rgb(200, 50, 50),
                        );
                    }
                    ui.painter().text(
                        close_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "X",
                        egui::FontId::proportional(14.0),
                        ui.style().visuals.text_color(),
                    );
                    if close_resp.clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            ui.add_space(4.0);
            ui.separator();
        });

        // --- Bottom panel: status bar ---
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status_text = if self.bot_core_exists {
                    "BOT核心: 存在"
                } else {
                    "BOT核心: 不存在"
                };
                ui.label(status_text);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("by Demon")
                            .size(11.0)
                            .color(egui::Color32::from_rgba_premultiplied(128, 128, 128, 120)),
                    );
                });
            });
        });

        // --- Central panel ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let style = ui.style_mut();
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);

            ui.add_space(6.0);

            // Target path row
            ui.horizontal(|ui| {
                ui.label("目标路径:");
                let avail = ui.available_width();
                ui.add(
                    egui::TextEdit::singleline(&mut self.target_path)
                        .desired_width((avail - 44.0).max(180.0)),
                );
                if ui.button("📁").clicked() {
                    self.browse_folder();
                }
            });

            ui.add_space(4.0);

            // Load button (full width, centered)
            let load_enabled = self.state != LoadState::Loading;
            let load_text = if self.state == LoadState::Loading {
                "加载中..."
            } else {
                "加载"
            };
            ui.vertical_centered(|ui| {
                let w = ui.available_width();
                if ui
                    .add_enabled(
                        load_enabled,
                        egui::Button::new(egui::RichText::new(load_text).size(15.0))
                            .min_size(egui::vec2(w, 38.0)),
                    )
                    .clicked()
                {
                    self.do_load();
                }
            });

            ui.add_space(4.0);
            ui.separator();

            // Logs (fill remaining space)
            ui.label("── 日志 ──");
            let log_h = ui.available_height() - 6.0;
            egui::ScrollArea::vertical()
                .max_height(log_h.max(60.0))
                .max_width(f32::INFINITY)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for log in &self.logs {
                        ui.add(egui::Label::new(log).extend());
                    }
                });

            if self.state != LoadState::Loading {
                self.bot_core_exists = extract::check_bot_core(
                    &self.config.get_target_path(&self.exe_dir),
                );
            }
        });
    }
}

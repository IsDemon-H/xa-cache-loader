use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Result of extraction progress callback
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ExtractProgress {
    Message(String),
    Progress(f32), // 0.0 ~ 1.0
    Done,
}

/// Extract a .zip file to target directory
pub fn extract_zip(
    zip_path: &Path,
    target_dir: &Path,
    progress_cb: impl Fn(ExtractProgress),
) -> Result<(), String> {
    let file = fs::File::open(zip_path).map_err(|e| format!("无法打开zip文件: {}", e))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("无法解析zip文件: {}", e))?;

    let total = archive.len();
    fs::create_dir_all(target_dir).map_err(|e| format!("无法创建目标目录: {}", e))?;

    for i in 0..total {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("读取zip条目失败: {}", e))?;

        let name = entry.name().to_string();
        // Skip directories
        if entry.is_dir() {
            continue;
        }

        let out_path = target_dir.join(&name);
        // Create parent dirs if needed
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("无法创建目录: {}", e))?;
        }

        // Write the file
        let mut out_file =
            fs::File::create(&out_path).map_err(|e| format!("无法创建文件 {}: {}", name, e))?;

        let mut buf = Vec::new();
        entry
            .read_to_end(&mut buf)
            .map_err(|e| format!("读取zip内容失败: {}", e))?;
        out_file
            .write_all(&buf)
            .map_err(|e| format!("写入文件 {} 失败: {}", name, e))?;

        let progress = (i + 1) as f32 / total as f32;
        progress_cb(ExtractProgress::Progress(progress));
    }

    Ok(())
}

/// Extract a .7z file to target directory
pub fn extract_7z(
    sevenz_path: &Path,
    target_dir: &Path,
    progress_cb: impl Fn(ExtractProgress),
) -> Result<(), String> {
    fs::create_dir_all(target_dir).map_err(|e| format!("无法创建目标目录: {}", e))?;

    sevenz_rust::decompress_file(sevenz_path, target_dir)
        .map_err(|e| format!("7z解压失败: {}", e))?;

    progress_cb(ExtractProgress::Progress(1.0));
    Ok(())
}

/// Delete a file
pub fn delete_file(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("无法删除文件 {}: {}", path.display(), e))?;
    }
    Ok(())
}

/// Move a file to a subdirectory (creating if needed)
pub fn move_file_to_dir(src: &Path, dest_dir: &Path) -> Result<(), String> {
    if !src.exists() {
        return Err(format!("文件不存在: {}", src.display()));
    }
    let file_name = src
        .file_name()
        .ok_or_else(|| "无法获取文件名".to_string())?;
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("无法创建目录 {}: {}", dest_dir.display(), e))?;
    let dest = dest_dir.join(file_name);
    fs::rename(src, &dest).map_err(|e| {
        format!(
            "无法移动文件 {} -> {}: {}",
            src.display(),
            dest.display(),
            e
        )
    })?;
    Ok(())
}

/// Delete all files in the saves subdirectory
pub fn clear_saves_dir(target_dir: &Path) -> Result<(), String> {
    let saves_dir = target_dir.join("saves");
    if !saves_dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&saves_dir).map_err(|e| format!("无法读取saves目录: {}", e))? {
        let entry = entry.map_err(|e| format!("读取saves条目失败: {}", e))?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path)
                .map_err(|e| format!("无法删除文件 {}: {}", path.display(), e))?;
        }
    }
    Ok(())
}

/// Check if bot_5.dll exists in the target directory
pub fn check_bot_core(target_dir: &Path) -> bool {
    target_dir.join("bot_5.dll").exists()
}

/// Get exe directory
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

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

// ============================================================================
// Network / Download / MD5
// ============================================================================

/// Version info fetched from remote repo
#[derive(Debug, Clone, serde::Deserialize)]
pub struct VersionInfo {
    pub md5: String,
    pub size: u64,
    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub filename: String,
}

pub const VERSION_URL: &str = "https://gitee.com/dog176/xaupload/raw/master/version.json";
pub const DOWNLOAD_URL: &str = "https://gitee.com/dog176/xaupload/raw/master/Xa缓存.zip";

/// Check network connectivity by HEAD request, return latency in ms
pub fn check_network() -> Result<u64, String> {
    let start = std::time::Instant::now();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("创建网络客户端失败: {}", e))?;

    client
        .head("https://gitee.com")
        .send()
        .map_err(|e| format!("网络不可达: {}", e))?;

    Ok(start.elapsed().as_millis() as u64)
}

/// Calculate MD5 hash of a file
pub fn calc_file_md5(path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    let digest = md5::compute(&buf);
    Ok(format!("{:x}", digest))
}

/// Fetch version info from remote repo
pub fn fetch_version_info() -> Result<VersionInfo, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let resp = client
        .get(VERSION_URL)
        .send()
        .map_err(|e| format!("获取版本信息失败: {}", e))?;

    let body = resp
        .text()
        .map_err(|e| format!("读取版本信息失败: {}", e))?;

    let v: VersionInfo =
        serde_json::from_str(&body).map_err(|e| format!("解析版本信息失败: {}", e))?;

    Ok(v)
}

/// Format bytes to human-readable string
pub fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.0}KB", bytes as f64 / 1024.0)
    } else {
        format!("{}B", bytes)
    }
}

/// Format a 20-char progress bar
pub fn format_progress(downloaded: u64, total: u64) -> String {
    let pct = if total > 0 {
        (downloaded as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };
    let filled = if total > 0 {
        (downloaded as f64 / total as f64 * 20.0) as usize
    } else {
        0
    };
    let filled = filled.min(20);
    let empty = 20 - filled;
    format!(
        "[{}>{}] {}%  ({}/ {})",
        "=".repeat(filled.saturating_sub(1)).to_string() + if filled > 0 { "" } else { "" },
        "-".repeat(if filled > 0 { empty.max(1) - if filled == 20 { 1 } else { 0 } } else { empty.max(1) }),
        pct,
        format_size(downloaded),
        format_size(total),
    )
}

/// Download file with progress callback
pub fn download_file(
    url: &str,
    dest: &Path,
    progress_cb: impl Fn(u64, u64),
) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("创建下载客户端失败: {}", e))?;

    let mut resp = client
        .get(url)
        .send()
        .map_err(|e| format!("下载请求失败: {}", e))?;

    let total = resp.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut buf = Vec::new();

    // Read in chunks, report progress
    let mut chunk = [0u8; 8192];
    loop {
        let n = resp
            .read(&mut chunk)
            .map_err(|e| format!("下载读取失败: {}", e))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        downloaded += n as u64;
        progress_cb(downloaded, total);
    }

    // Write to destination
    let mut file =
        fs::File::create(dest).map_err(|e| format!("无法创建文件 {}: {}", dest.display(), e))?;
    file.write_all(&buf)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

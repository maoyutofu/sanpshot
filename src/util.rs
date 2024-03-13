use std::{fs, io, path::Path};

use chrono::Local;

/// 生成文件，文件名由本地时间组成
pub fn generate_file_name() -> String {
    let local = Local::now();
    format!("{}.jpg", local.format("%Y-%m-%d-%H-%M-%S"))
}

/// 文件全路径组装
pub fn file_path_join(a: &str, b: &str) -> Option<String> {
    let path_buf = Path::new(a).join(b);
    match path_buf.to_str() {
        Some(p) => Some(String::from(p)),
        None => None,
    }
}

/// 判断路径不存在就创建
pub fn create_dir(path_str: &str) -> io::Result<()> {
    let p = Path::new(path_str);
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            return Ok(fs::create_dir_all(parent)?);
        }
    }
    Ok(())
}

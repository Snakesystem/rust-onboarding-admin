use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

/// 📂 Inisialisasi folder dan file log berdasarkan konfigurasi `.env`
pub fn init_log() -> std::io::Result<String> {

    let log_dir = env::var("./log-snake").unwrap_or_else(|_| "log-snake".to_string());

    if !std::path::Path::new(&log_dir).exists() {
        std::fs::create_dir_all(&log_dir)?;
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    let log_file = format!("{}/log-{}.rs", log_dir, date);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)?;

    let log_msg = format!("//[{}] Application started\n", Local::now().format("%Y-%m-%d %H:%M:%S"));
    file.write_all(log_msg.as_bytes())?;

    Ok(log_file)
}

/// 📝 Fungsi untuk menulis log
pub fn write_log(level: &str, message: &str) {
    let log_dir = env::var("./log-snake").unwrap_or_else(|_| "log-snake".to_string());
    let date = Local::now().format("%Y-%m-%d").to_string();
    let log_file = format!("{}/log-{}.rs", log_dir, date); // testing untuk file rs sebagai log

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
        let log_msg = format!("//[{}] [{}] {}\n", Local::now().format("%Y-%m-%d %H:%M:%S"), level, message);
        let _ = file.write_all(log_msg.as_bytes());
    }
}
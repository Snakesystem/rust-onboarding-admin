
use base64::{decode, DecodeError};
use chrono::Local;
use std::{env, fs, io::Write, path::PathBuf};

use super::generic_service::GenericService;
pub struct FileService;

impl FileService {

    pub fn save_base64_image(email: &str, base64_str: &str, file_name: &str) -> Result<String, String> {
        let path_env = env::var("PATH_ASSET").expect("PATH_ASSET harus diatur");
        // Cek dan ekstrak format file dari base64
        let (format, base64_data) = if let Some((meta, data)) = base64_str.split_once(",") {
            if let Some(ext) = meta.strip_prefix("data:image/").and_then(|m| m.split_once(";").map(|(f, _)| f)) {
                (ext, data)
            } else {
                return Err("Invalid base64 format".to_string());
            }
        } else {
            return Err("Invalid base64 input".to_string());
        };
        
        // Decode base64
        let image_bytes = decode(base64_data).map_err(|e: DecodeError| format!("Base64 decode error: {}", e))?;
        
        // Format tanggal
        let date_str = Local::now().format("%Y%m%d").to_string();
        
        // Generate random string untuk nama folder
        let random_str = GenericService::random_string_by_suffix(30, &email, &file_name);
        
        // Buat path folder dan file
        let save_folder = PathBuf::from(format!("{}/{}{}", path_env, random_str, date_str));
        let save_path = save_folder.join(format!("{}.{}", file_name, format));
        
        // Buat folder jika belum ada
        fs::create_dir_all(&save_folder).map_err(|e| format!("Failed to create folder: {}", e))?;
        
        // Simpan file
        let mut file = fs::File::create(&save_path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(&image_bytes).map_err(|e| format!("Failed to write file: {}", e))?;
        
        // Return path yang akan disimpan ke database
        Ok(format!("{}/{}.{}", format!("{}{}", random_str, date_str), file_name, format))
    }

}
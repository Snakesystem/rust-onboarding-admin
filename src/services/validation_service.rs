pub mod validator {
    use std::collections::HashMap;
    use base64::{engine::general_purpose, Engine as _}; // Pake Engine
    use chrono::{DateTime, Utc};
    use image::ImageFormat;
    use regex::Regex;
    use validator::{ValidationError, ValidationErrors};

    pub fn required(value: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            let mut error = ValidationError::new("required");
            error.message = Some("This field is required".into());
            return Err(error);
        }
        Ok(())
    }

    pub fn required_int(value: i32) -> Result<(), ValidationError> {
        if value == 0 {
            let mut error = ValidationError::new("required");
            error.message = Some("This field hand not null or 0".into());
            return Err(error);
        }
        Ok(())
    }  

    pub fn required_datetime(value: &DateTime<Utc>) -> Result<(), ValidationError> {
        if value.to_string().trim().is_empty() {
            let mut error = ValidationError::new("required");
            error.message = Some("This field is required".into());
            return Err(error);
        }
        Ok(())
    }    

    pub fn valid_name(value: &str) -> Result<(), ValidationError> {
        let email_regex = Regex::new(r"^[a-zA-Z ]+$")
            .map_err(|_| ValidationError::new("invalid_regex"))?;

        if !email_regex.is_match(value) {
            let mut error = ValidationError::new("invalid_email");
            error.message = Some("Format name value has not number".into());
            return Err(error);
        }
        Ok(())
    }

    pub fn valid_password(value: &str) -> Result<(), ValidationError> {
        let password_regex = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$")
            .map_err(|_| ValidationError::new("invalid_regex"))?;

        if !password_regex.is_match(value) {
            let mut error = ValidationError::new("invalid_password");
            error.message = Some("Required character number and text".into());
            return Err(error);
        }
        Ok(())
    }

    pub fn valid_phone_number(value: &str) -> Result<(), ValidationError> {
        let phone_regex = Regex::new(r"^\d{10,15}$")
            .map_err(|_| ValidationError::new("invalid_regex"))?;

        if !phone_regex.is_match(value) {
            let mut error = ValidationError::new("invalid_phone");
            error.message = Some("Valus has number from 10-15 length".into());
            return Err(error);
        }
        Ok(())
    }

    pub fn valid_number_card(value: &str) -> Result<(), ValidationError> {
        let phone_regex = Regex::new(r"^[0-9]*$")
            .map_err(|_| ValidationError::new("invalid_regex"))?;

        if !phone_regex.is_match(value) {
            let mut error = ValidationError::new("invalid_number_card");
            error.message = Some("Value has number format".into());
            return Err(error);
        }
        Ok(())
    }

    /// Fungsi validasi Base64 Image
    pub fn validate_base64_image(value: &str) -> Result<(), ValidationError> {
        let base64_cleaned = value
        .split(',')
        .last()
        .unwrap_or("")
        .trim();

        let decoded = general_purpose::STANDARD.decode(base64_cleaned).map_err(|_| {
            let mut error = ValidationError::new("invalid_base64");
            error.message = Some("Base64 tidak valid".into());
            error
        })?;
        
        // Cek ukuran maksimum (misal 5MB)
        if decoded.len() > 5 * 1024 * 1024 {
            let mut error = ValidationError::new("file_too_large");
            error.message = Some("Ukuran file maksimal 5MB".into());
            return Err(error);
        }
    
        // Cek apakah Base64 ini benar-benar gambar
        let format = image::guess_format(&decoded).map_err(|_| {
            let mut error = ValidationError::new("invalid_image");
            error.message = Some("File bukan gambar valid".into());
            error
        })?;
    
        // Cek apakah formatnya JPEG, PNG, atau WebP
        match format {
            ImageFormat::Jpeg | ImageFormat::Png | ImageFormat::WebP => Ok(()),
            _ => {
                let mut error = ValidationError::new("unsupported_format");
                error.message = Some("Format gambar harus JPEG, PNG, atau WebP".into());
                Err(error)
            }
        }
    }

    pub fn format_validation_errors(errors: &ValidationErrors) -> HashMap<String, String> {
        let mut formatted_errors = HashMap::new();
    
        for (field, field_errors) in errors.field_errors() {
            if let Some(error) = field_errors.first() {
                let error_message = match error.code.as_ref() {
                    "required" => format!("{} is required", capitalize(&field)),
                    _ => error.message.clone().unwrap_or_else(|| "Invalid value".into()).to_string(),
                };
    
                formatted_errors.insert(field.to_string(), error_message);
            }
        }
    
        formatted_errors
    }
    
    // Helper untuk kapitalisasi huruf pertama
    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }    
    
}
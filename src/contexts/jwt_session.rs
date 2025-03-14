use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Utc};
use super::model::WebUser;

const SECRET_KEY: &[u8] = b"supersecretkey"; // 🔥 Ganti dengan key yang lebih aman!

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub result: bool,
    pub auth_usernid: i32,
    pub email: String,
    pub mobile_phone: String,
    pub disabled_login: bool,
    pub expired_token: i64,
    pub expired_date: String,
    pub register_date: DateTime<Utc>,
    pub exp: usize,
    pub picture: Option<String>,
}

impl Claims {
    pub fn new(user: WebUser) -> Self {
        let expired_token = Utc::now() + Duration::days(7); // Token berlaku 7 hari
        let expired_date = expired_token.format("%Y-%m-%d %H:%M:%S").to_string();
        let exp = expired_token.timestamp() as usize; // ⏳ Set exp untuk validasi JWT

        Self {
            result: true,
            email: user.email,
            expired_token: expired_token.timestamp(),
            expired_date,
            disabled_login: user.disabled_login,
            auth_usernid: user.auth_usernid,
            mobile_phone: user.mobile_phone,
            picture: user.picture,
            register_date: user.register_date,
            exp, // 🔥 Tambahkan ke struct
        }
    }
}

// 🔥 Generate JWT Token
pub fn create_jwt(user: WebUser) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user); // 🔥 Clone user di sini
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )?;
    Ok(token)
}

// 🔥 Validate JWT Token
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            let claims = token_data.claims;
            let now = Utc::now().timestamp() as usize;

            if claims.exp < now {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature,
                ));
            }

            Ok(claims)
        }
        Err(err) => {
            println!("❌ JWT Validation Error: {:?}", err);
            Err(err)
        }
    }
}


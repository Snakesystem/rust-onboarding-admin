use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;
use crate::services::validation_service::validator::{
    required, valid_phone_number, valid_name, valid_number_card, required_int,
    required_datetime, validate_base64_image, valid_password
}; 
// pub struct DateTimeConverter;

// impl DateTimeConverter {
//     pub fn from_string(date_str: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
//         let format = "%Y-%m-%d %H:%M:%S";
//         chrono::NaiveDateTime::parse_from_str(date_str, format)
//     }
// }

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(required, email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_password"))]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(required, email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_password"))]
    pub password: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_phone_number"))]
    pub mobile_phone: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_name"))]
    pub full_name: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub bank_account_number: Option<String>,
    #[validate(custom(function = "required"))]
    pub bank_name: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_name"))]
    pub bank_account_holder: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub question_rdn: i32,
    #[serde(default)]
    pub sales: i32,
    #[serde(default)]
    pub referal: String,
    pub client_category: u8,
    #[serde(default)]
    pub app_ipaddress: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct DataPribadiRequest {
    #[validate(custom(function = "required"), email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_phone_number"))]
    pub mobile_phone: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_name"))]
    pub full_name: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_name"))]
    pub mother_name: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"), length(min = 15, message = "Minimum 15 characters"))]
    pub idcard_number: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub nationality: i32,
    #[validate(custom(function = "required_int"))]
    pub sex: i32,
    #[validate(custom(function = "required_int"))]
    pub residence_status: i32,
    #[validate(custom(function = "required_int"))]
    pub beneficiary_owner: i32,
    #[validate(custom(function = "required"))]
    pub birth_place: Option<String>,
    #[validate(custom(function = "required_datetime"))]
    #[serde(deserialize_with  = "deserialize_date_only")]
    pub birth_date: Option<DateTime<Utc>>,
    #[validate(custom(function = "required"))]
    pub birth_country: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub religion: i32,
    #[validate(custom(function = "required_int"))]
    pub marital_status: i32,
    #[validate(custom(function = "required_int"))]
    pub education: i32,
    pub copy_id: Option<bool>,
    #[validate(custom(function = "required_datetime"))]
    #[serde(deserialize_with  = "deserialize_date_only")]
    pub idcard_expireddate  : Option<DateTime<Utc>>,
    #[validate(custom(function = "required_int"))]
    pub idcard_country  : i32,

    #[validate(custom(function = "validate_base64_image"))]
    pub idcard_file: String,
    #[validate(custom(function = "validate_base64_image"))]
    pub selfie_file: String,
    #[validate(custom(function = "validate_base64_image"))]
    pub signature_file: String,

    // #region ID CARD FIELD INFORMAATION
    #[validate(custom(function = "required_int"))]
    pub idcard_city: i32,
    #[validate(custom(function = "required"))]
    pub idcard_district: Option<String>,
    #[validate(custom(function = "required"))]
    pub idcard_subdistrict: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub idcard_rt: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"),)]
    pub idcard_rw: Option<String>,
    #[validate(custom(function = "required"))]
    pub idcard_address: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"),)]
    pub idcard_zipcode: Option<String>,
    // #endregion

    // #region DOMICILE FIELD INFORMAATION
    #[validate(custom(function = "required_int"))]
    pub domicile_city: i32,
    #[validate(custom(function = "required"))]
    pub domicile_district: Option<String>,
    #[validate(custom(function = "required"))]
    pub domicile_subdistrict: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub domicile_rt: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"),)]
    pub domicile_rw: Option<String>,
    #[validate(custom(function = "required"))]
    pub domicile_address: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"),)]
    pub domicile_zipcode: Option<String>,
    // #endregion
}

#[derive(Debug, Deserialize, Validate)]
pub struct DataBankRequest {
    #[validate(custom(function = "required_int"))]
    pub question_rdn: i32,
    #[validate(custom(function = "required"))]
    pub bank_name: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_name"))]
    pub bank_account_holder: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub bank_account_number: Option<String>,
    #[validate(custom(function = "required"))]
    pub bank_branch: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DataPekerjaanRequest {
    #[validate(custom(function = "required"))]
    pub company_name: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub company_city: i32,
    #[validate(custom(function = "required"))]
    pub company_address: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub company_zipcode: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub question_npwp: i32,
    #[validate(custom(function = "required"))]
    pub npwp_reason: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub npwp_number: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub fund_source: i32,
    pub fund_source_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub occupation: i32,
    pub occupation_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub nature_bussiness: i32,
    pub nature_bussiness_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub position: i32,
    pub position_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub income_peranum: i32,
    #[validate(custom(function = "required"))]
    pub spouse_name: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub spouse_relationship: i32,
    #[validate(custom(function = "required_int"))]
    pub spouse_occupation: i32,
    pub spouse_occupation_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub spouse_fund_source: i32,
    pub spouse_fund_source_text: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub spouse_position: i32,
    #[validate(custom(function = "required_int"))]
    pub spouse_income_peranum: i32,
    #[validate(custom(function = "required_int"))]
    pub spouse_nature_bussiness: i32,
    #[validate(custom(function = "required"))]
    pub spouse_company_name: Option<String>,
    #[validate(custom(function = "required_int"))]
    pub spouse_company_city: i32,
    #[validate(custom(function = "required"))]
    pub spouse_company_address: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_number_card"))]
    pub spouse_company_zipcode: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DataPendukungRequest {
    pub question_1: bool,
    pub question_1text: Option<String>,

    pub question_2: bool,
    pub question_2text: Option<String>,

    pub question_3: bool,
    pub question_3text: Option<String>,

    pub question_4: bool,
    pub question_4text: Option<String>,

    pub question_5: bool,
    pub question_5text: Option<String>,

    pub question_6: bool,
    pub question_6text: Option<String>,

    #[validate(custom(function = "required_int"))]
    pub investment_objective: i32,
    
    #[validate(custom(function = "required_int"))]
    pub risk: i32,
    
    #[validate(custom(function = "required"))]
    pub question_fatca: Option<String>,
    
    #[validate(custom(function = "required"))]
    pub fatca_1: Option<String>,
    
    #[validate(custom(function = "required"))]
    pub fatca_2: Option<String>,
    
    #[validate(custom(function = "required"))]
    pub fatca_3: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(required, email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(required, email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(custom(function = "required"), custom(function = "valid_password"))]
    pub password: Option<String>,
    pub reset_password_key: String
}

#[derive(Debug, Serialize)]
pub struct ActionResult<T, E> {
    pub result: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<E>,
}

// Implementasi Default
impl<T, E> Default for ActionResult<T, E> {
    fn default() -> Self {
        Self {
            result: false, // Default-nya false
            message: String::new(),
            data: None,
            error: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct WebUser {
    pub auth_usernid: i32,
    pub email: String,
    pub mobile_phone: String,
    pub disabled_login: bool,
    pub picture: Option<String>,
    #[serde(serialize_with = "serialize_datetime")]
    pub register_date: chrono::DateTime<Utc>
}

#[derive(Debug, Serialize, Clone)]
pub struct UserInfo {
    pub autonid: i32,
    pub stage: i16,
    pub client_id: String,
    pub cif_id: String,
    pub is_revised: bool,
    pub is_rejected: bool,
    pub is_finished: bool,
    pub account_status: String,
    pub mobile_phone: String,
    pub email: String,
    pub full_name: String,
    pub spouse_relationship: i32,
    pub spouse_name: String,
    pub mother_name: String,
    pub nationality: i32,
    pub idcard_country: i32,
    pub idcard_number: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub idcard_expire_date: chrono::DateTime<Utc>,
    pub sex: i32,
    pub birth_place: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub birth_date: chrono::DateTime<Utc>,
    pub birth_country: i32,
    pub religion: i32,
    pub marital_status: i32,
    pub education: i32,
    pub idcard_city: i32,
    pub idcard_district: String,
    pub idcard_subdistrict: String,
    pub idcard_rt: String,
    pub idcard_rw: String,
    pub idcard_zipcode: String,
    pub idcard_address: String,
    pub copy_id: bool,
    pub domicile_city: i32,
    pub domicile_district: String,
    pub domicile_subdistrict: String,
    pub domicile_rt: String,
    pub domicile_rw: String,
    pub domicile_address: String,
    pub domicile_zipcode: String,
    pub question_rdn: i32,
    pub bank_name: String,
    pub bank_branch: String,
    pub bank_account_number: String,
    pub bank_account_holder: String,
    pub question_npwp: i32,
    pub npwp_number: String,
    pub npwp_reason: String,
    pub company_name: String,
    pub fund_source: i32,
    pub fund_source_text: String,
    pub occupation: i32,
    pub occupation_text: String,
    pub nature_bussiness: i32,
    pub nature_bussiness_text: String,
    pub position: i32,
    pub position_text: String,
    pub income_peranum: i32,
    pub question_1: bool,
    pub question_1text: String,
    pub question_2: bool,
    pub question_2text: String,
    pub question_3: bool,
    pub question_3text: String,
    pub question_4: bool,
    pub question_4text: String,
    pub question_5: bool,
    pub question_5text: String,
    pub question_6: bool,
    pub question_6text: String,
    pub invesment_objective: i32,
    pub risk: i32,
    pub question_fatca: String,
    pub fatca_1: String,
    pub fatca_2: String,
    pub fatca_3: String,
    pub spouse_income_peranum: i32,
    pub spouse_occupation: i32,
    pub spouse_occupation_text: String,
    pub spouse_position: i32,
    pub spouse_nature_bussiness: i32,
    pub spouse_fund_source: i32,
    pub spouse_fund_source_text: String,
    pub spouse_company_name: String,
    pub spouse_company_city: i32,
    pub spouse_company_address: String,
    pub spouse_company_zipcode: String,
    pub idcard_file: String,
    pub selfie_file: String,
    pub signature_file: String,
    pub npwp_file: String,
    pub sales: i32,
    pub company_city: i32,
    pub company_address: String,
    pub company_zipcode: String,
    pub beneficiary_owner: i32,
    pub residence_status: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Company {
    pub company_id: String,
    pub company_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListData {
    pub data_id: i32,
    pub code: String,
    pub description: String,
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

fn deserialize_date_only<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    if let Some(date) = date_str {
        let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)?;
        let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        return Ok(Some(datetime));
    }
    Ok(None)
}
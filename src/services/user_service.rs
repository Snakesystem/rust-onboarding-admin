use std::collections::HashMap;
use actix_web::web;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use chrono::{NaiveDateTime, TimeZone, Utc};
use tokio_stream::StreamExt;
use tiberius::QueryStream;

use crate::contexts::{
    connection::Transaction, 
    jwt_session::Claims, 
    model::{ActionResult, DataBankRequest, DataPekerjaanRequest, DataPendukungRequest, DataPribadiRequest, UserInfo}
};

pub struct UserService;

impl UserService {
    pub async fn get_user_info(connection: web::Data<Pool<ConnectionManager>>, session: Claims) -> ActionResult<Vec<UserInfo>, String> {
        let mut result: ActionResult<Vec<UserInfo>, String> = ActionResult::default();

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(r#"
                        SELECT 
                            A.*,
                            B.ContactPersonName, 
                            B.ContactPersonRelation,
                            B.CIFInvestorBeneficiaryOwner,
                            B.ResidencyNStatus
                            FROM UserKyc A
                        LEFT JOIN TableRequest B ON B.WebCIFNID = A.AutoNID 
                        WHERE A.AutoNID = @P1"#, &[&session.auth_usernid]).await;
                match query_result {
                    Ok(mut rows) => {
                        let mut cities: Vec<UserInfo> = Vec::new(); // ⬅️ Tampung semua data
                        while let Some(query_item) = rows.try_next().await.unwrap_or(None) {
                            if let Some(row) = query_item.as_row() { // ⬅️ Konversi QueryItem menjadi Row
                                cities.push(UserInfo {
                                    autonid: row.get::<i32, _>("AutoNID").unwrap_or(0),
                                    stage: row.get::<i16, _>("Stage").unwrap_or(0),
                                    client_id: row.get::<&str, _>("CIFID").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    cif_id: row.get::<&str, _>("ClientID").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    is_revised: row.get("IsRevised").unwrap_or(false),
                                    is_rejected: row.get("IsRejected").unwrap_or(false),
                                    is_finished: row.get("IsFinished").unwrap_or(false),
                                    account_status: row.get::<&str, _>("AccountStatus").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    mobile_phone: row.get::<&str, _>("MobilePhone").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    email: row.get::<&str, _>("Email").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    full_name: row.get::<&str, _>("Fullname").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_relationship: row.get::<i32, _>("SpouseRelationship").unwrap_or(0),
                                    spouse_name: row.get::<&str, _>("SpouseName").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    mother_name: row.get::<&str, _>("MotherName").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    nationality: row.get::<i32, _>("Nationality").unwrap_or(0),
                                    idcard_country: row.get::<i32, _>("IDCardCountry").unwrap_or(0),
                                    idcard_number: row.get::<&str, _>("IDcardNumber").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_expire_date: row
                                        .get::<NaiveDateTime, _>("IDCardExpiredDate")
                                        .map(|dt| dt.and_utc()) // 🔥 Konversi ke DateTime<Utc>
                                        .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()), // Default jika kosong
                                    sex: row.get::<i32, _>("IDCardCountry").unwrap_or(0),
                                    birth_place: row.get::<&str, _>("BirthPlace").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    birth_date: row
                                        .get::<NaiveDateTime, _>("BirthDate")
                                        .map(|dt| dt.and_utc()) // 🔥 Konversi ke DateTime<Utc>
                                        .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()), // Default jika kosong
                                    birth_country: row.get::<i32, _>("BirthCountry").unwrap_or(0),
                                    religion: row.get::<i32, _>("Religion").unwrap_or(0),
                                    marital_status: row.get::<i32, _>("MaritalStatus").unwrap_or(0),
                                    education: row.get::<i32, _>("Education").unwrap_or(0),
                                    idcard_city: row.get::<i32, _>("IDCardCity").unwrap_or(0),
                                    idcard_district: row.get::<&str, _>("IDCardDistrict").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_subdistrict: row.get::<&str, _>("IDCardSubdistrict").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_rt: row.get::<&str, _>("IDCardRT").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_rw: row.get::<&str, _>("IDCardRW").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_zipcode: row.get::<&str, _>("IDCardZipcode").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_address: row.get::<&str, _>("IDCardAddress").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    copy_id: row.get("CopyID").unwrap_or(false),
                                    domicile_city: row.get::<i32, _>("DomicileCity").unwrap_or(0),
                                    domicile_district: row.get::<&str, _>("DomicileDistrict").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    domicile_subdistrict: row.get::<&str, _>("DomicileSubdistrict").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    domicile_rt: row.get::<&str, _>("DomicileRT").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    domicile_rw: row.get::<&str, _>("DomicileRW").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    domicile_zipcode: row.get::<&str, _>("DomicileZipcode").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    domicile_address: row.get::<&str, _>("DomicileAddress").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_rdn: row.get::<i32, _>("QuestionRDN").unwrap_or(0),
                                    bank_name: row.get::<&str, _>("BankName").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    bank_branch: row.get::<&str, _>("BankBranch").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    bank_account_holder: row.get::<&str, _>("BankAccountHolder").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    bank_account_number: row.get::<&str, _>("BankAccountNumber").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_npwp: row.get::<i32, _>("QuestionNPWP").unwrap_or(0),
                                    npwp_number: row.get::<&str, _>("NPWPNumber").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    npwp_reason: row.get::<&str, _>("NPWPReason").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    company_name: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    fund_source: row.get::<i32, _>("").unwrap_or(0),
                                    fund_source_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    occupation: row.get::<i32, _>("").unwrap_or(0),
                                    occupation_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    nature_bussiness: row.get::<i32, _>("").unwrap_or(0),
                                    nature_bussiness_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    position: row.get::<i32, _>("").unwrap_or(0),
                                    position_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    income_peranum: row.get::<i32, _>("").unwrap_or(0),
                                    question_1: row.get("").unwrap_or(false),
                                    question_1text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_2: row.get("").unwrap_or(false),
                                    question_2text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_3: row.get("").unwrap_or(false),
                                    question_3text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_4: row.get("").unwrap_or(false),
                                    question_4text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_5: row.get("").unwrap_or(false),
                                    question_5text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    question_6: row.get("").unwrap_or(false),
                                    question_6text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    invesment_objective: row.get::<i32, _>("").unwrap_or(0),
                                    risk: row.get::<i32, _>("").unwrap_or(0),
                                    question_fatca: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    fatca_1: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    fatca_2: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    fatca_3: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_income_peranum: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_occupation: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_occupation_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_position: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_nature_bussiness: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_fund_source: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_fund_source_text: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_company_name: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_company_city: row.get::<i32, _>("").unwrap_or(0),
                                    spouse_company_address: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    spouse_company_zipcode: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    idcard_file: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    selfie_file: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    signature_file: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    npwp_file: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    sales: row.get::<i32, _>("").unwrap_or(0),
                                    company_city: row.get::<i32, _>("").unwrap_or(0),
                                    company_address: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    company_zipcode: row.get::<&str, _>("").map_or_else(|| "".to_string(), |s| s.to_string()),
                                    beneficiary_owner: row.get::<i32, _>("").unwrap_or(0),
                                    residence_status: row.get::<i32, _>("").unwrap_or(0),

                                });

                                result.result = true;
                                result.message = "Cities list retrieved successfully".to_string();
                                result.data = Some(cities.clone()); // ⬅️ Simpan array hasil query
                            } else {
                                result.message = "No Cities found".to_string();
                                result.data = [].to_vec().into();
                            }
                        }
                        
                        return result;
                    }
                    Err(e) => {
                        result.message = "Internal Server Error".to_string();
                        result.error = Some(e.to_string());
                        return result;
                    }
                }
            }
            Err(e) => {
                result.error = Some(e.to_string());
                return result;
            }
        }
    }

    pub async fn save_data_pribadi(connection: web::Data<Pool<ConnectionManager>>, request: DataPribadiRequest, session: Claims) -> ActionResult<HashMap<String, String>, String> {

        let mut result: ActionResult<HashMap<String, String>, String> = ActionResult::default();
        let current_stage: i32 = 1;

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AutoNID, Stage 
                    FROM UserKyc 
                    WHERE AutoNID = @P1"#, &[&session.auth_usernid]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            let stage: i32 = row.get("Stage").unwrap_or(0);
                            let auto_nid: i32 = row.get("AutoNID").unwrap_or(0);

                            if stage < current_stage {
                                result.message = "Stage has ben first or 1".to_owned();
                                return result;
                            }

                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                        r#"UPDATE [dbo].[UserKyc]
                                            SET [Stage] = @P1, [Fullname] = @P2, [Nationality] = @P3, [IDCardNumber] = @P4, [IDCardExpireDate] = @P5, 
                                                [Sex] = @P6,[BirthDate] = @P7, [BirthPlace] = @P8, [BirthCountry] = @P9, [MotherName] = @P10, [Religion] = @P11, 
                                                [MaritalStatus] = @P12,[Education] = @P13, [IDCardCity] = @P14, [IDCardDistrict] = @P15, 
                                                [IDCardSubdistrict] = @P16, [IDCardRT] = @P17, [IDCardRW] = @P18, [IDCardZipcode] = @P19, 
                                                [IDCardAddress] = @P20, [CopyID] = @P21, [DomicileCity] = @P22, [DomicileDistrict] = @P23, 
                                                [DomicileSubdistrict] = @P24, [DomicileRT] = @P25, [DomicileRW] = @P26, [DomicileZipcode] = @P27, 
                                                [DomicileAddress] = @P28, [IDCardFile] = @P29, [SelfieFile] = @P30, [SignatureFile] = @P31, IDCardCountry = @P32
                                            WHERE AutoNID = @P33"#,
                                                &[
                                                    &2i32,
                                                    &request.full_name,
                                                    &request.nationality,
                                                    &request.idcard_number,
                                                    &request.idcard_expireddate,
                                                    &request.sex,
                                                    &request.birth_date,
                                                    &request.birth_place,
                                                    &request.birth_country,
                                                    &request.mother_name,
                                                    &request.religion,
                                                    &request.marital_status,
                                                    &request.education,
                                                    &request.idcard_city,
                                                    &request.idcard_district,
                                                    &request.idcard_subdistrict,
                                                    &request.idcard_rt,
                                                    &request.idcard_rw,
                                                    &request.idcard_zipcode,
                                                    &request.idcard_address,
                                                    &request.copy_id,
                                                    &request.domicile_city,
                                                    &request.domicile_district,
                                                    &request.domicile_subdistrict,
                                                    &request.domicile_rt,
                                                    &request.domicile_rw,
                                                    &request.domicile_zipcode,
                                                    &request.domicile_address,
                                                    &"request.idcard_file",
                                                    &"request.selfie_file",
                                                    &"request.signature_file",
                                                    &request.idcard_country,
                                                    &auto_nid
                                                ],
                                            ).await {
                                                result.error = Some(format!("Fauled: {:?}", err));
                                                return result;
                                            }
                                        }
                                        None => {
                                            result.error = Some("Failed to get database connection".into());
                                            return result;
                                        }
                                    }
                    
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                                r#"UPDATE [dbo].[TableRequest] SET [ResidencyNStatus] = @P1, [CIFInvestorBeneficiaryOwner] = @P2 WHERE [AutoNID] = @P3"#,
                                                &[&request.residence_status, &request.beneficiary_owner, &auto_nid],
                                            ).await {
                                                result.error = Some(format!("Failed to insert TableRequest: {:?}", err));
                                                return result;
                                            }
                                        }
                                        None => {
                                            result.error = Some("Failed to get database connection".into());
                                            return result;
                                        }
                                    }

                                    // 🔵 Commit transaksi
                                    if let Err(err) = trans.commit().await {
                                        result.error = Some(format!("Failed to commit transaction: {:?}", err));
                                        return result;
                                    }
                    
                                    result.result = true;
                                    result.message = "Update personal data successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                    },
                }
            },
            Err(err) => {
                result.error = format!("Internal Server error: {:?}", err).into();
                return result;
            }, 
        }

        return result;
    }

    pub async fn save_data_bank(connection: web::Data<Pool<ConnectionManager>>, request: DataBankRequest, session: Claims) -> ActionResult<HashMap<String, String>, String> {

        let mut result: ActionResult<HashMap<String, String>, String> = ActionResult::default();
        let current_stage: i32 = 2;

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AutoNID, Stage 
                    FROM UserKyc 
                    WHERE AutoNID = @P1"#, &[&session.auth_usernid]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            let stage: i32 = row.get("Stage").unwrap_or(0);
                            let auto_nid: i32 = row.get("AutoNID").unwrap_or(0);

                            if stage < current_stage {
                                result.message = "Stage has ben second or 2".to_owned();
                                return result;
                            }

                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                            r#"UPDATE [dbo].[UserKYC]
                                                SET [Stage] = @P1, [QuestionRDN] = @P2, [BankName] = @P3, [BankAccountHolder] = @P4, [BankAccountNumber] = @P5, [BankBranch] = @P6
                                            WHERE AutoNID = @P7"#,
                                                &[
                                                    &3i32,
                                                    &request.question_rdn,
                                                    &request.bank_name,
                                                    &request.bank_account_holder,
                                                    &request.bank_account_number,
                                                    &request.bank_branch,
                                                    &auto_nid
                                                ],
                                            ).await {
                                                result.error = Some(format!("Fauled: {:?}", err));
                                                return result;
                                            }
                                        }
                                        None => {
                                            result.error = Some("Failed to get database connection".into());
                                            return result;
                                        }
                                    }

                                    // 🔵 Commit transaksi
                                    if let Err(err) = trans.commit().await {
                                        result.error = Some(format!("Failed to commit transaction: {:?}", err));
                                        return result;
                                    }
                    
                                    result.result = true;
                                    result.message = "Update personal data successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                    },
                }
            },
            Err(err) => {
                result.error = format!("Internal Server error: {:?}", err).into();
                return result;
            }, 
        }

        return result;
    }

    pub async fn save_data_pekerjaan(connection: web::Data<Pool<ConnectionManager>>, request: DataPekerjaanRequest, session: Claims) -> ActionResult<HashMap<String, String>, String> {

        let mut result: ActionResult<HashMap<String, String>, String> = ActionResult::default();
        let curent_stage: i32 = 3;

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AutoNID, Stage 
                    FROM UserKyc 
                    WHERE AutoNID = @P1"#, &[&session.auth_usernid]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            let stage: i32 = row.get("Stage").unwrap_or(0);
                            let auto_nid: i32 = row.get("AutoNID").unwrap_or(0);

                            if stage < curent_stage {
                                result.message = "Stage has ben second or 3".to_owned();
                                return result;
                            }

                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                        r#"UPDATE [dbo].[UserKyc]  
                                                SET [Stage] = @P1, [QuestionNPWP] = @P2, [NPWPNumber] = @P3, [NPWPReason] = @P4, [CompanyAddress] = @P5,   
                                                [CompanyName] = @P6, [Fundsource] = @P7, [Occupation] = @P8, [OccupationText] = @P9,   
                                                [NatureOfBusiness] = @P10, [NatureOfBusinessText] = @P11, [Position] = @P12,  
                                                [PositionText] = @P13, [IncomePerAnnum] = @P14, [FundsourceText] = @P15, [SpouseIncomePerAnnum] = @P16,  
                                                [SpouseOccupation] = @P17, [SpousePosition] = @P18,  
                                                [SpouseNatureOfBusiness] = @P19,[SpouseCompanyName] = @P20,[SpouseCompanyCity] = @P21,  
                                                [SpouseCompanyZipcode] = @P22,[SpouseCompanyAddress] = @P23,  
                                                [SpouseFundSource] = @P24,[SpouseFundSourceText] = @P25, [SpouseOccupationText] = @P26, [SpouseName] = @P27,
                                                [SpouseRelationship] = @P28
                                                WHERE AutoNID = @P29"#,
                                                &[
                                                    &4i32,
                                                    &request.question_npwp,
                                                    &request.npwp_number,
                                                    &request.npwp_reason,
                                                    &request.company_address,
                                                    &request.company_name,
                                                    &request.fund_source,
                                                    &request.occupation,
                                                    &request.occupation_text,
                                                    &request.nature_bussiness,
                                                    &request.nature_bussiness_text,
                                                    &request.position,
                                                    &request.position_text,
                                                    &request.income_peranum,
                                                    &request.fund_source_text,
                                                    &request.spouse_income_peranum,
                                                    &request.spouse_occupation,
                                                    &request.spouse_position,
                                                    &request.spouse_nature_bussiness,
                                                    &request.spouse_company_name,
                                                    &request.spouse_company_city,
                                                    &request.spouse_company_zipcode,
                                                    &request.spouse_company_address,
                                                    &request.spouse_fund_source,
                                                    &request.spouse_fund_source_text,
                                                    &request.spouse_occupation_text,
                                                    &request.spouse_name,
                                                    &request.spouse_relationship,
                                                    &auto_nid
                                                ],
                                            ).await {
                                                result.error = Some(format!("Fauled: {:?}", err));
                                                return result;
                                            }
                                        }
                                        None => {
                                            result.error = Some("Failed to get database connection".into());
                                            return result;
                                        }
                                    }

                                    // 🔵 Commit transaksi
                                    if let Err(err) = trans.commit().await {
                                        result.error = Some(format!("Failed to commit transaction: {:?}", err));
                                        return result;
                                    }
                    
                                    result.result = true;
                                    result.message = "Update personal data successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                    },
                }
            },
            Err(err) => {
                result.error = format!("Internal Server error: {:?}", err).into();
                return result;
            }, 
        }

        return result;
    }

    pub async fn save_data_pendukung(connection: web::Data<Pool<ConnectionManager>>, request: DataPendukungRequest, session: Claims) -> ActionResult<HashMap<String, String>, String> {

        let mut result: ActionResult<HashMap<String, String>, String> = ActionResult::default();
        let current_stage: i32 = 4;

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AutoNID, Stage 
                    FROM UserKyc 
                    WHERE AutoNID = @P1"#, &[&session.auth_usernid]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            let stage: i32 = row.get("Stage").unwrap_or(0);
                            let auto_nid: i32 = row.get("AutoNID").unwrap_or(0);

                            if stage < current_stage {
                                result.message = "Stage has ben second or 4".to_owned();
                                return result;
                            }

                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                        r#"UPDATE [dbo].[UserKyc]  
                                                SET [Stage] = @P1, [Question1] = @P2, [Question1Text] = @P3,   
                                                [Question2] = @P4, [Question2Text] = @P5,   
                                                [Question3] = @P6, [Question3Text] = @P7,   
                                                [Question4] = @P8, [Question4Text] = @P9, [Question5] = @P10,  
                                                [Question5Text] = @P11, [Question6] = @P12,   
                                                [Question6Text] = @P13, [InvestmentObjectives] = @P14, [Risk] = @P15,  
                                                [QuestionFATCA] = @P16, [FATCA1] = @P17, [FATCA2] = @P18, [FATCA3] = @P19, IsFinished = 1 
                                                WHERE AutoNID = @P20"#,
                                                &[
                                                    &5i32,
                                                    &request.question_1,
                                                    &request.question_1text,
                                                    &request.question_2,
                                                    &request.question_2text,
                                                    &request.question_3,
                                                    &request.question_3text,
                                                    &request.question_4,
                                                    &request.question_4text,
                                                    &request.question_5,
                                                    &request.question_5text,
                                                    &request.question_6,
                                                    &request.question_6text,
                                                    &request.investment_objective,
                                                    &request.risk,
                                                    &request.question_fatca,
                                                    &request.fatca_1,
                                                    &request.fatca_2,
                                                    &request.fatca_3,
                                                    &auto_nid
                                                ],
                                            ).await {
                                                result.error = Some(format!("Fauled: {:?}", err));
                                                return result;
                                            }
                                        }
                                        None => {
                                            result.error = Some("Failed to get database connection".into());
                                            return result;
                                        }
                                    }

                                    // 🔵 Commit transaksi
                                    if let Err(err) = trans.commit().await {
                                        result.error = Some(format!("Failed to commit transaction: {:?}", err));
                                        return result;
                                    }
                    
                                    result.result = true;
                                    result.message = "Update personal data successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                    },
                }
            },
            Err(err) => {
                result.error = format!("Internal Server error: {:?}", err).into();
                return result;
            }, 
        }

        return result;
    }
}
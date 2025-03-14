use actix_web::web;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use chrono::{NaiveDateTime, TimeZone, Utc};
use tiberius::QueryStream;
use crate::contexts::{
    connection::Transaction, 
    crypto::encrypt_text, 
    model::{ActionResult, ChangePasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest, WebUser}
};
use super::generic_service::GenericService;

pub struct AuthService;

impl AuthService {
    pub async fn login(connection: web::Data<Pool<ConnectionManager>>,request: LoginRequest) -> ActionResult<WebUser, String> {
        
        let mut result: ActionResult<WebUser, String> = ActionResult::default();
        let enc_password = encrypt_text(request.password.unwrap_or_default());

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AuthUserNID, Email, Handphone, disableLogin, Picture, RegisterDate FROM AuthUser 
                    WHERE Email = @P1 AND Password = @P2"#, &[&request.email, &enc_password]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            result.result = true;
                            result.message = format!("Welcome {}", request.email.unwrap_or_default());
                            result.data = Some(WebUser{
                                auth_usernid: row.get("AuthUserNID").unwrap_or(0),
                                email: row.get::<&str, _>("Email").map_or_else(|| "".to_string(), |s| s.to_string()),
                                mobile_phone: row.get::<&str, _>("Handphone").map_or_else(|| "".to_string(), |s| s.to_string()),
                                disabled_login: row.get("disableLogin").unwrap_or(false),
                                picture: Some(row.get::<&str, _>("Picture").map_or_else(|| "".to_string(), |s| s.to_string())),
                                register_date: row
                                    .get::<NaiveDateTime, _>("RegisterDate")
                                    .map(|dt| dt.and_utc()) // 🔥 Konversi ke DateTime<Utc>
                                    .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap()), // Default jika kosong
                            }); 

                            return result;
                        } else {
                            result.message = format!("No user found for email: {}", request.email.unwrap_or_default());
                            return result;
                        } 
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                        return result;
                    },
                }
            },
            Err(err) => {
                result.error = format!("Internal Server error: {:?}", err).into();
                return result;
            }, 
        }
    }

    pub async fn register(connection: web::Data<Pool<ConnectionManager>>, request: RegisterRequest) -> ActionResult<(), String> {
        
        let mut result: ActionResult<(), String> = ActionResult::default();
        let enc_password = encrypt_text(request.password.unwrap_or_default());

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT Email FROM AuthUser WHERE Email = @P1"#, &[&request.email]
                ).await;
        
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            if row.get::<&str, _>("Email").is_some() {
                                result.result = false;
                                result.error = Some("Email already exists".into());
                                return result;
                            }
                        }
                    }
                    Err(err) => {
                        result.error = Some(format!("Query error: {}", err));
                        return result;
                    }
                }
            }
            Err(err) => {
                result.error = Some(format!("Database connection error: {}", err));
                return result;
            }
        }
        match Transaction::begin(&connection).await {
            Ok(trans) => {
                let auto_nid: i32;

                // 🔴 Scope pertama: Insert ke UserKyc
                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        match conn.query(
                            r#"INSERT INTO [dbo].[UserKyc] 
                            ([Email],[MobilePhone],[Fullname],[BankAccountNumber],[BankAccountHolder],
                            [QuestionRDN],[Sales],[BankName],[Stage],[CIFNID],[ChangeNID],[PendingCIFNID],
                            [IsRejected],[IsFinished],[IsRevised],[IsImported],[SaveTime],[LastUpdate],[SaveIpAddress])
                            OUTPUT INSERTED.AutoNID
                            VALUES
                            (@P1,@P2,@P3,@P4,@P5,@P6,@P7,@P8,@P9,@P10,@P11,@P12,@P13,@P14,@P15,@P16,@P17,@P18,@P19)"#,
                            &[
                                &request.email, &request.mobile_phone, &request.full_name,
                                &request.bank_account_number, &request.bank_account_holder,
                                &request.question_rdn, &request.sales, &request.bank_name,
                                &1i32, &0i32, &0i32, &0i32, &false, &false, &false, &false,
                                &chrono::Utc::now(), &chrono::Utc::now(), &request.app_ipaddress,
                            ],
                        ).await {
                            Ok(rows) => {
                                auto_nid = match rows.into_row().await {
                                    Ok(Some(row)) => row.get("AutoNID").unwrap_or(0),
                                    _ => {
                                        result.error = Some("Failed to get AutoNID from UserKyc".into());
                                        return result;
                                    }
                                };
                            }
                            Err(err) => {
                                result.error = Some(format!("Failed to insert UserKyc: {:?}", err));
                                return result;
                            }
                        }
                    }
                    None => {
                        result.error = Some("Failed to get connection from pool".into());
                        return result;
                    }
                }

                // 🔴 Scope kedua: Insert ke AuthUser
                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        if let Err(err) = conn.execute(
                            r#"INSERT INTO [dbo].[AuthUser] 
                            ([WebCIFNID],[Email],[Handphone],[ActivateCode],[Password],[RegisterDate],
                            [disableLogin],[OTPGeneratedLink],[OTPGeneratedLinkDate],[Picture],[Sub], [ClientNCategory])
                            VALUES (@P1,@P2,@P3,@P4,@P5,@P6,@P7,@P8,@P9,@P10,@P11,@P12)"#,
                            &[
                                &auto_nid, &request.email, &request.mobile_phone, &GenericService::random_string(20),
                                &enc_password, &chrono::Utc::now(), &true,
                                &GenericService::random_string(70), &chrono::Utc::now(),
                                &"", &"", &request.client_category,
                            ],
                        ).await {
                            result.error = Some(format!("Failed to insert AuthUser: {:?}", err));
                            return result;
                        }
                    }
                    None => {
                        result.error = Some("Failed to get database connection".into());
                        return result;
                    }
                }

                // 🔴 Scope ketiga: Insert ke TableRequest
                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        if let Err(err) = conn.execute(
                            r#"INSERT INTO [dbo].[TableRequest] ([WebCIFNID], [Referal]) VALUES (@P1, @P2)"#,
                            &[&auto_nid, &request.referal],
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
                result.message = "User registered successfully".to_string();
            }
            Err(err) => {
                result.error = Some(format!("Failed to start transaction: {:?}", err));
            }
        }

        return result;
    }

    pub async fn activation_user(connection: web::Data<Pool<ConnectionManager>>, otp_link: String) -> ActionResult<(), String> {

        let mut result: ActionResult<(), String> = ActionResult::default();

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AuthUserNID 
                    FROM AuthUser 
                    WHERE OTPGeneratedLink = @P1"#, &[&otp_link]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                                r#"UPDATE [dbo].[AuthUser]
                                                    set [OTPGeneratedLink] = @P2, [disableLogin] = @P3,
                                                    [ActivateTime] = @P4
                                                    WHERE AuthUserNID = @P1"#,
                                                &[
                                                    &row.get("AuthUserNID").unwrap_or(0),
                                                    &otp_link,
                                                    &false,
                                                    &chrono::Utc::now(),
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
                                    result.message = "Activation successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                            return result;
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                        return result;
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

    pub async  fn forget_password(connection: web::Data<Pool<ConnectionManager>>, request: ResetPasswordRequest) -> ActionResult<(), String> {

        let mut result: ActionResult<(), String> = ActionResult::default();

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AuthUserNID 
                    FROM AuthUser 
                    WHERE Email = @P1"#, &[&request.email]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                                r#"UPDATE [dbo].[AuthUser]
                                                    SET [ResetPasswordKey] = @P2, [ResetPasswordFlag] = @P3, [ResetPasswordDate] = @P4
                                                    WHERE AuthUserNID = @P1"#,
                                                &[
                                                    &row.get("AuthUserNID").unwrap_or(0),
                                                    &GenericService::random_string(70),
                                                    &true,
                                                    &chrono::Utc::now(),
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
                                    result.message = "Reset password successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                            return result;
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                        return result;
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

    pub async  fn change_password(connection: web::Data<Pool<ConnectionManager>>, request: ChangePasswordRequest) -> ActionResult<(), String> {

        let mut result: ActionResult<(), String> = ActionResult::default();
        let enc_password = encrypt_text(request.password.unwrap_or_default());

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query(
                    r#"SELECT AuthUserNID 
                    FROM AuthUser 
                    WHERE Email = @P1 and ResetPasswordKey = @P2"#, &[&request.email, &request.reset_password_key]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            match Transaction::begin(&connection).await {
                                Ok(trans) => {
                                    // 🔴 Scope ketiga: Insert ke TableRequest
                                    match trans.conn.lock().await.as_mut() {
                                        Some(conn) => {
                                            if let Err(err) = conn.execute(
                                                r#"UPDATE [dbo].[AuthUser]
                                                    set [ResetPasswordKey] = @P2, [ResetPasswordFlag] = @P3, [Password] = @P4
                                                    WHERE AuthUserNID = @P1"#,
                                                &[
                                                    &row.get("AuthUserNID").unwrap_or(0),
                                                    &request.reset_password_key,
                                                    &true,
                                                    &enc_password,
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
                                    result.message = "Change password successfully".to_string();
                                }
                                Err(err) => {
                                    result.error = Some(format!("Failed to start transaction: {:?}", err));
                                }
                            }
                    
                        } else {
                            result.message = format!("No user found for email");
                            return result;
                        }
                    },
                    Err(err) => {
                        result.error = format!("Query execution failed: {:?}", err).into();
                        return result;
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
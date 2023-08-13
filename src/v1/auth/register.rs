use std::time::SystemTime;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_client_ip::ClientIp;
use chrono::Duration;
use crypto::{password::make_hash, token::generate_token};
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};
use sqlx::query;

use crate::{
    datetime_utc_from_timestamp,
    v1::{ApiError, ResultResponse},
    AppState, DateTimeUtc,
};

use api_types::{v1::register::*, Snowflake};

pub async fn get_registration_info() -> Json<RegistrationPrerequisites> {
    Json(RegistrationPrerequisites {
        hcaptcha_sitekey: env!("HCAPTCHA_SITEKEY").to_string(),
    })
}

pub async fn make_registration(
    State(app_state): State<AppState>,
    ClientIp(ip): ClientIp,
    Json(request): Json<RegistrationRequest>,
) -> ResultResponse<Json<RegistrationResponse>> {
    // Try to validate the hCaptcha response

    async fn process_captcha(request: &RegistrationRequest) -> Result<(), RegistrationResponse> {
        let client = HcaptchaClient::new();
        let captcha_response = HcaptchaCaptcha::new(&request.hcaptcha_response);
        let captcha_response = match captcha_response {
            Ok(resp) => resp,
            Err(e) => {
                return Err(RegistrationResponse::HcaptchaFailure {
                    error: format!("Could not parse incoming CAPTCHA response: {e}"),
                })
            }
        };

        let request = match HcaptchaRequest::new(env!("HCAPTCHA_SECRET_KEY"), captcha_response) {
            Ok(r) => r,
            Err(e) => {
                return Err(RegistrationResponse::HcaptchaFailure {
                    error: format!("Could not construct CAPTCHA verify request: {e}"),
                })
            }
        };

        let response = match client.verify_client_response(request).await {
            Ok(resp) => resp,
            Err(e) => {
                return Err(RegistrationResponse::HcaptchaFailure {
                    error: format!("Could not fetch result of CAPTCHA verification: {e}"),
                })
            }
        };

        if !response.success() {
            return Err(RegistrationResponse::HcaptchaFailure {
                error: format!("CAPTCHA challenge was not successful"),
            });
        }

        Ok(())
    }

    match process_captcha(&request).await {
        Ok(_) => {}
        Err(e) => return Ok(Json(e)),
    };

    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let email_str = request.email.to_string();
    let pending_registration = query!(
        "SELECT * FROM registration WHERE email=? AND expires_unix_time > ?",
        email_str,
        now
    )
    .fetch_optional(&app_state.db)
    .await?;
    match pending_registration {
        Some(pending_registration) => Ok(Json(RegistrationResponse::PendingRegistrationExists {
            id: pending_registration.id.into(),
        })),
        None => {
            // NB: even if the username/email is already taken, we still create a registration request.
            // This is to prevent enumeration attacks.
            // make_registration will still create it,
            // but it will not send the confirmation email;
            // instead, it will send an email to the owner of the existing account.
            // This email will not contain the confirmation token,
            // thus making it impossible to confirm the registration.
            let registration_id = {
                let snowflake = Snowflake::new().await;
                let now = snowflake.timestamp();
                const TOKEN_LENGTH: u16 = 32;
                let token = generate_token(TOKEN_LENGTH);
                let expires = (now + registration_expiration()).timestamp();
                let resend_after = (now + email_resend_after()).timestamp();
                let password_hash = make_hash(&request.password);
                let ip_str = ip.to_string();
                let email_str = request.email.to_string();
                query!("INSERT INTO registration (id, username, email, password_hash, created_by_ip, expires_unix_time, confirm_token, email_resend_after_unix_time) values (?,?,?,?,?,?,?,?)",
                    snowflake,
                    request.username,
                    email_str,
                    password_hash,
                    ip_str,
                    expires,
                    token,
                    resend_after
                ).execute(&app_state.db).await?;

                // Check if there is a user already registered for this username, and for this address.
                // If there is a user for this email address, send a message to that email address.
                // If there is a user for this username,

                let user = query!("SELECT * FROM user WHERE email=?", email_str)
                    .fetch_optional(&app_state.db)
                    .await?;
                if user.is_none() {
                    let message = mail::templates::registration::make_registration_confirm_email(
                        request.email,
                        &token,
                    );
                    let status = mail::delivery::send_message(message).await;
                    match status {
                        Ok(_) => snowflake,
                        Err(error) => {
                            tracing::error!("Error while sending message: {:?}", error);
                            // TODO: figure out if error is temporary or permanent, and maybe error out if permanent
                            snowflake
                        }
                    }
                } else {
                    // This is where we need to send the email that does not contain the token.
                    let message = mail::templates::registration::make_duplicate_registration_email(
                        request.email,
                    );
                    let status = mail::delivery::send_message(message).await;
                    match status {
                        Ok(_) => snowflake,
                        Err(error) => {
                            tracing::error!("Error while sending message: {:?}", error);
                            // TODO: figure out if error is temporary or permanent, and maybe error out if permanent
                            snowflake
                        }
                    }
                }
            };
            Ok(Json(RegistrationResponse::Ok {
                id: registration_id,
            }))
        }
    }
}

pub fn registration_expiration() -> Duration {
    Duration::hours(8)
}

fn email_resend_after() -> Duration {
    Duration::minutes(5)
}

pub async fn get_registration(
    State(app_state): State<AppState>,
    Path(registration_id): Path<Snowflake>,
) -> ResultResponse<Json<PendingRegistration>> {
    let now = DateTimeUtc::from(SystemTime::now()).timestamp();
    let pending_registration = query!(
        "SELECT * FROM registration WHERE id=? AND expires_unix_time > ?",
        registration_id,
        now
    )
    .fetch_optional(&app_state.db)
    .await?;
    match pending_registration {
        None => Err(ApiError::NotFound.into()),
        Some(reg) => {
            let email = match reg.email.parse() {
                Ok(email) => email,
                Err(err) => {
                    tracing::error!(
                        "Error parsing email {:?} from registration {} from database?! {}",
                        reg.email,
                        registration_id,
                        err
                    );
                    return Err(ApiError::UnexpectedError(format!("Could not parse email as lettre::Address in registration {} (email is {:?})", registration_id, reg.email)).into());
                }
            };
            let reg = PendingRegistration {
                username: reg.username,
                email,
                can_resend_email_after: datetime_utc_from_timestamp(
                    reg.email_resend_after_unix_time,
                ),
                expires_after: datetime_utc_from_timestamp(reg.expires_unix_time),
            };
            Ok(Json(reg))
        }
    }
}

pub async fn resend_confirm_email(
    State(app_state): State<AppState>,
    Path(reg_id): Path<Snowflake>,
) -> ResultResponse<(StatusCode, Json<ResendConfirmationResponse>)> {
    // Get the registration by id, bail early if not found
    let pending_registration = query!("SELECT * FROM registration WHERE id=?", reg_id)
        .fetch_optional(&app_state.db)
        .await?;
    let registration = match pending_registration {
        None => return Err(ApiError::NotFound.into()),
        Some(reg) => reg,
    };

    // Check if we are now after the retry time
    let now: DateTimeUtc = SystemTime::now().into();
    if registration.email_resend_after_unix_time > now.timestamp() {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ResendConfirmationResponse::TooEarly),
        ));
    }

    // Try resending the email
    let resend_result = {
        let db = &app_state.db;
        let email: lettre::Address = registration
            .email
            .parse()
            .expect("Failed to parse email from database?!");
        let message = mail::templates::registration::make_registration_confirm_email(
            email.clone(),
            &registration.confirm_token,
        );
        let status = mail::delivery::send_message(message).await;

        let now: DateTimeUtc = SystemTime::now().into();
        let resend_after = (now + email_resend_after()).timestamp();

        match status {
            Ok(_) => {
                // Update the repeat timer
                query!(
                    "UPDATE registration SET email_resend_after_unix_time=? WHERE id=?",
                    resend_after,
                    reg_id
                )
                .execute(db)
                .await?;
                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Failed to send repeat confirmation for email {:?} for registration {}: {:?}",
                    email,
                    registration.id,
                    error
                );

                // Update the repeat timer anyway (TODO: figure out whether the error is on our side, and do not resend if so)
                query!(
                    "UPDATE registration SET email_resend_after_unix_time=? WHERE id=?",
                    resend_after,
                    reg_id
                )
                .execute(db)
                .await?;

                Err(error)
            }
        }
    };
    match resend_result {
        Err(email_err) => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendConfirmationResponse::SendingError {
                    error: email_err.to_string(),
                }),
            ))
        }
        Ok(_) => return Ok((StatusCode::OK, Json(ResendConfirmationResponse::Ok))),
    };
}

use actix_web::{dev::ServiceRequest, web::Data, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{errors::ServerError, middleware::Claims, Config};

pub async fn auth(req: ServiceRequest, creds: BearerAuth) -> Result<ServiceRequest, Error> {
    let app_data: &Data<Config> = req.app_data().ok_or(ServerError::InternalError {
        message: "No app data".into(),
    })?;

    let config = app_data.lock().await;
    let password = config.server.password.clone();

    let key = DecodingKey::from_secret(password.as_ref());

    let claims = decode::<Claims>(creds.token(), &key, &Validation::default())
        .map_err(|e| {
            log::info!("JWT validation failed: {}", e);
            ServerError::Unauthorized {
                message: "Unauthorized".into(),
            }
        })?
        .claims;

    if !config.paired_devices.contains(&claims.dev) {
        return Err(ServerError::Unauthorized {
            message: "Unauthorized".into(),
        }
        .into());
    }

    std::mem::drop(config);

    Ok(req)
}

use actix_web::{dev::ServiceRequest, web::Data, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{errors::ServerError, middleware::Claims, Config};

pub async fn jwt(req: ServiceRequest, creds: BearerAuth) -> Result<ServiceRequest, Error> {
    let app_data: &Data<Config> = req.app_data().ok_or(ServerError::InternalError {
        message: "No app data".into(),
    })?;

    let config = app_data.lock().await;
    let password = config.server.password.clone();
    std::mem::drop(config);

    let key = DecodingKey::from_secret(password.as_ref());

    match decode::<Claims>(creds.token(), &key, &Validation::default()) {
        Ok(_) => Ok(req),
        Err(e) => {
            log::info!("JWT validation failed: {}", e);
            Err(ServerError::Unauthorized {
                message: "Unauthorized".into(),
            }
            .into())
        }
    }
}

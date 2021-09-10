use actix_web::{web::Data, HttpResponse, Result};

use crate::{errors::ServerError, utils::aes::encrypt, Config};

pub async fn unpair(config: Data<Config>, payload: String) -> Result<HttpResponse, ServerError> {
    let mut config = config.lock().await;

    config.unpair_device(payload).await.map_err(|e| {
        log::error!("Failed to unpair: {}", e);
        ServerError::PairingError {
            message: e.to_string(),
        }
    })?;

    Ok(HttpResponse::Ok().body(encrypt("OK", &config.server.password)?))
}

#[cfg(test)]
mod tests {

    use std::{path::Path, sync::Arc};

    use actix_web::web::Data;
    use tokio::sync::Mutex;

    use crate::{
        test::utils::{create_test_config_with_paired_device, create_test_folder, TEST_DEV},
        Result,
    };

    #[tokio::test]
    async fn unpair_route() -> Result<()> {
        let path = Path::new("unpair_route");

        let tempdir = create_test_folder(path)?;

        let config = create_test_config_with_paired_device(tempdir.path(), TEST_DEV).await?;

        let app_data = Data::new(Arc::new(Mutex::new(config.clone())));

        // Ok
        let resp = super::unpair(app_data, TEST_DEV.into()).await;
        assert!(resp.is_ok());

        tempdir.close()?;

        Ok(())
    }
}

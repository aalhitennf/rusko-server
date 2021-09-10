use actix_web::{web::Data, HttpResponse, Result};

use crate::{errors::ServerError, utils::aes::encrypt, Config};

pub async fn pair(config: Data<Config>, payload: String) -> Result<HttpResponse, ServerError> {
    let mut config = config.lock().await;

    config.pair_device(payload).await.map_err(|e| {
        log::error!("Failed to pair: {}", e);
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
        test::utils::{
            create_test_config_with_paired_device, create_test_folder, res_body_to_string, TEST_DEV,
        },
        Result,
    };

    // TODO validate that the device id gets written in file
    #[tokio::test]
    async fn pair_route() -> Result<()> {
        let path = Path::new("pair_route");

        let tempdir = create_test_folder(path)?;

        let config = create_test_config_with_paired_device(tempdir.path(), TEST_DEV).await?;

        let app_data = Data::new(Arc::new(Mutex::new(config.clone())));

        // Fail with same device
        let resp = super::pair(app_data.clone(), TEST_DEV.into()).await;
        assert!(resp.is_err());

        // Ok
        let resp = super::pair(
            app_data.clone(),
            "1234567890123456ql4zjjcgFVYA5EC0uCkLS6cZ7MRHGGmucfXUwala5E4=".into(),
        )
        .await;
        assert!(resp.is_ok());

        tempdir.close()?;

        // TODO Fail with invalid data
        // let resp = super::pair(app_data, "1234567890123456DJ9kOsDWtFUbiYN44NjDaZjvjChNm0L9+Y01z6jkCr0=".into()).await;
        // assert!(resp.is_ok());

        let body = res_body_to_string(resp.unwrap())?;

        assert!(body.len() == 40);

        Ok(())
    }
}

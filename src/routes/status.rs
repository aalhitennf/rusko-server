use actix_web::{web::Data, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::{errors::ServerError, utils::aes::encrypt, Config};

#[derive(Deserialize, Serialize)]
struct ServerStatus {
    ok: bool,
}

pub async fn status(config: Data<Config>) -> Result<HttpResponse, ServerError> {
    let config = config.lock().await;

    let json_str = serde_json::to_string(&ServerStatus { ok: true }).map_err(|_| {
        ServerError::InternalError {
            message: "Failed to stringify json".into(),
        }
    })?;

    Ok(HttpResponse::Ok().body(encrypt(&json_str, &config.server.password)?))
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
    async fn status_route() -> Result<()> {
        let path = Path::new("status_route");

        let tempdir = create_test_folder(path)?;

        let config = create_test_config_with_paired_device(tempdir.path(), TEST_DEV).await?;

        let app_data = Data::new(Arc::new(Mutex::new(config.clone())));

        // Ok
        let resp = super::status(app_data).await;
        assert!(resp.is_ok());

        tempdir.close()?;

        Ok(())
    }
}

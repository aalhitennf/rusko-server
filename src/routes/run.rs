use actix_web::{web::Data, HttpResponse, Result};

use crate::{
    errors::ServerError,
    utils::aes::{decrypt, encrypt},
    Config,
};

pub async fn run(config: Data<Config>, input: String) -> Result<HttpResponse, ServerError> {
    let config = config.lock().await;

    if config
        .find_command(&decrypt(&input, &config.server.password)?)
        .ok_or(ServerError::BadRequest {
            message: "Command not found".into(),
        })?
        .execute()
    {
        Ok(HttpResponse::Ok().body(encrypt("OK", &config.server.password)?))
    } else {
        Ok(HttpResponse::BadRequest().body(encrypt("Failed", &config.server.password)?))
    }
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
    async fn run_route() -> Result<()> {
        let path = Path::new("run_route");

        let tempdir = create_test_folder(path)?;

        let config = create_test_config_with_paired_device(tempdir.path(), TEST_DEV).await?;

        let app_data = Data::new(Arc::new(Mutex::new(config.clone())));

        // Ok
        let resp = super::run(
            app_data.clone(),
            "1234567890123456S3pOoa8CpD8JjfZ/yQd6gQ==".into(),
        )
        .await;
        assert!(resp.is_ok());

        // Fail
        let resp = super::run(app_data, "1234567890123456stb91yZF2TbxxpHyzLCTnw==".into()).await;
        assert!(resp.is_err());

        tempdir.close()?;

        Ok(())
    }
}

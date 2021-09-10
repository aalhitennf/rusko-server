use actix_web::{web::Data, HttpResponse, Result};

use crate::{errors::ServerError, Config};

pub async fn commands(config: Data<Config>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().body(config.lock().await.commands_to_enc_str()?))
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

    #[tokio::test]
    async fn commands_route() -> Result<()> {
        let path = Path::new("commands_route");

        let tempdir = create_test_folder(path)?;

        let config = create_test_config_with_paired_device(tempdir.path(), TEST_DEV).await?;

        let app_data = Data::new(Arc::new(Mutex::new(config.clone())));

        let resp = super::commands(app_data).await?;

        tempdir.close()?;

        let body = res_body_to_string(resp)?;

        assert!(body.len() == 60);

        Ok(())
    }
}

use actix_multipart::Multipart;
use actix_web::{web::Data, HttpResponse, Result};
use futures::StreamExt;
use serde::Deserialize;

use crate::{
    errors::ServerError,
    utils::{
        aes::{decrypt, encrypt},
        write,
    },
    Config,
};

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub name: String,
    pub data: String,
}

// TODO Check for refactor
pub async fn upload(
    config: Data<Config>,
    mut payload: Multipart,
) -> Result<HttpResponse, ServerError> {
    let config = config.lock().await;
    let password = config.server.password.clone();
    let mut path = config.dirs.upload_folder.clone();

    std::mem::drop(config);

    let mut buffer: Vec<u8> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|_| ServerError::InternalError {
            message: "Connection lost".into(),
        })?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            buffer.append(&mut data.to_vec());
        }
    }

    let string_json = String::from_utf8(buffer).map_err(|_| ServerError::BadRequest {
        message: "Invalid utf8".into(),
    })?;

    let upload: FormData =
        serde_json::from_str(&decrypt(&string_json, &password)?).map_err(|_| {
            ServerError::BadRequest {
                message: "Failed to parse data".into(),
            }
        })?;

    path = path.join(&upload.name);

    let bytes = base64::decode(&upload.data).map_err(|_| ServerError::BadRequest {
        message: "Invalid base64".into(),
    })?;

    write::async_new(&bytes, &path)
        .await
        .map_err(|_| ServerError::InternalError {
            message: "Failed to save file".into(),
        })?;

    Ok(HttpResponse::Ok().body(encrypt("OK", &password)?))
}

// TODO Test

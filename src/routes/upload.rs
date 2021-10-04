use std::path::PathBuf;

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

// Get the multipart data and process it in thread
pub async fn upload(
    config: Data<Config>,
    mut payload: Multipart,
) -> Result<HttpResponse, ServerError> {
    let config = config.lock().await;
    let password = config.server.password.clone();
    let path = config.dirs.upload_folder.clone();

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
    
    let password_clone = password.clone();

    std::thread::spawn(move || handle_upload(buffer, &password_clone, path));

    Ok(HttpResponse::Ok().body(encrypt("OK", &password)?))
}

// Step by step
fn handle_upload(buffer: Vec<u8>, password: &str, mut path: PathBuf) {
    let string_json = String::from_utf8(buffer);

    if string_json.is_err() {
        log::error!("Upload failed: Invalid utf8");
        return;
    }

    let decrypted = decrypt(&string_json.unwrap(), password);

    if decrypted.is_err() {
        log::error!("Upload failed: Decryption error");
        return;
    }

    let upload = serde_json::from_str(&decrypted.unwrap());

    if upload.is_err() {
        log::error!("Upload failed: Failed to parse JSON");
        return;
    }

    let form_data: FormData = upload.unwrap();

    path = path.join(&form_data.name);

    let bytes = base64::decode(&form_data.data);

    if bytes.is_err() {
        log::error!("Upload failed: Invalid Base64");
        return;
    }

    let write_result = write::blocking::new(&bytes.unwrap(), &path);

    if write_result.is_err() {
        log::error!("Upload failed: Failed to write to disk");
        return;
    }

    log::info!("Upload file: OK - {}", path.display());
}


// TODO Test

use aes::Aes128;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{distributions::Alphanumeric, Rng};

use crate::errors::ServerError;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// Outputs base64 encoded string
// ? For now message doesn't need to be generic
// ? but keep it like this for the future
pub fn encrypt<T>(message: &T, key: &str) -> Result<String, ServerError>
where
    T: AsRef<[u8]> + ?Sized,
{
    let mut iv: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let cipher = Aes128Cbc::new_from_slices(key.as_ref(), iv.as_ref()).map_err(|e| {
        ServerError::EncryptError {
            message: e.to_string(),
        }
    })?;

    let ciphertext = cipher.encrypt_vec(message.as_ref());

    iv.push_str(&base64::encode(ciphertext));

    Ok(iv)
}

// Message must be base64 encoded string
// 40 is minimum length of any encoded message with 16 char
// iv at the start. 16 + 24 = 40.
pub fn decrypt(encrypted: &str, key: &str) -> Result<String, ServerError> {
    if encrypted.len() < 40 {
        return Err(ServerError::DecryptError {
            message: "Invalid payload: Too short".into(),
        });
    };

    let base64_decoded =
        base64::decode(&encrypted[16..]).map_err(|e| ServerError::DecryptError {
            message: e.to_string(),
        })?;

    let cipher =
        Aes128Cbc::new_from_slices(key.as_ref(), encrypted[..16].as_ref()).map_err(|e| {
            ServerError::DecryptError {
                message: e.to_string(),
            }
        })?;

    let ciphertext =
        cipher
            .decrypt_vec(&base64_decoded)
            .map_err(|e| ServerError::DecryptError {
                message: e.to_string(),
            })?;

    String::from_utf8(ciphertext).map_err(|e| ServerError::DecryptError {
        message: e.to_string(),
    })
}

use crate::{
    input::{ENIGO, INPUT_MAP},
    utils::aes::decrypt,
};

use enigo::{Key, KeyboardControllable};

#[derive(Debug)]
struct InputData(i32, Key);

// TODO Ugly
// * Data needs to contain 4 parts separated with comma
// * If not, discard it by settings type to -1
impl InputData {
    fn from_str(value: &str, password: &str) -> Self {
        let mut input = InputData(-1, Key::Alt);

        let parts: Vec<&str> = value.split(',').collect();

        if parts.len() != 4 {
            return input;
        }

        // ?
        if let Ok(decrypted) = decrypt(parts[2], password) {
            input.0 = parts[1].parse::<i32>().unwrap();

            let chars: Vec<char> = decrypted.chars().collect();

            if chars.len() > 1 {
                if let Some(key) = INPUT_MAP.get(decrypted.as_str()) {
                    input.1 = *key;
                }
            } else {
                input.1 = Key::Layout(chars[0]);
            }
        };

        input
    }
}

pub fn handle(data: &str, password: &str) {
    let data = InputData::from_str(data, password);

    if data.0 == 6 {
        ENIGO.lock().unwrap().key_click(data.1);
    } else {
        log::error!("InputHandler: Invalid data {:?}", data);
    }
}

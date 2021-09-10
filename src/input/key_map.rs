use std::collections::HashMap;

use enigo::Key;
use once_cell::sync::Lazy;

pub static INPUT_MAP: Lazy<HashMap<&'static str, Key>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Enter", Key::Return);
    map.insert("Backspace", Key::Backspace);
    map
});

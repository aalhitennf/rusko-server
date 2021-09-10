use crate::input::ENIGO;

use enigo::{MouseButton, MouseControllable};

#[derive(Debug)]
struct InputData(i32, i32, i32);

impl InputData {
    fn from_str(data: &str) -> Self {
        let parts: Vec<&str> = data.split(',').collect();

        let mut input = InputData(0, 0, 0);

        if parts.len() == 4 {
            input.0 = parts[1].parse::<i32>().unwrap();
            input.1 = parts[2].parse::<i32>().unwrap();
            input.2 = parts[3].parse::<i32>().unwrap();
        } else {
            input.0 = -1;
        }

        input
    }
}

pub fn handle(data: &str) {
    let data = InputData::from_str(data);

    match data.0 {
        -1 => log::error!("InputHandler: Invalid data {:?}", data),
        1 => ENIGO.lock().unwrap().mouse_move_relative(data.1, data.2),
        2 => ENIGO.lock().unwrap().mouse_click(MouseButton::Left),
        3 => ENIGO.lock().unwrap().mouse_click(MouseButton::Right),
        4 => {
            ENIGO.lock().unwrap().mouse_scroll_y(data.1);
            ENIGO.lock().unwrap().mouse_scroll_x(data.2);
        }
        5 => match data.1 {
            0 => ENIGO.lock().unwrap().mouse_up(MouseButton::Left),
            1 => ENIGO.lock().unwrap().mouse_down(MouseButton::Left),
            _ => log::error!("WebSocket client is providing incorrect data: {:?}", data),
        },
        _ => (),
    }
}

use actix::{Actor, StreamHandler};
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use enigo::Enigo;
use once_cell::sync::Lazy;

use crate::{errors::ServerError, input::data_handlers, Config};

pub static ENIGO: Lazy<std::sync::Mutex<Enigo>> = Lazy::new(|| std::sync::Mutex::new(Enigo::new()));

#[derive(Debug)]
struct InputHandler {
    password: String,
}

impl Actor for InputHandler {
    type Context = ws::WebsocketContext<Self>;
}

// * Data types
// * 0 = numeric data
// * 1 = string data
// * 9 = handshake

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for InputHandler {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let data_type = get_data_type(&text);

                match data_type {
                    0 => data_handlers::numeric::handle(&text),
                    1 => data_handlers::char::handle(&text, &self.password),
                    9 => {
                        log::info!("WebSocket connected");
                        if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
                            if session_type == "wayland" {
                                ctx.close(Some(ws::CloseReason {
                                    code: ws::CloseCode::Unsupported,
                                    description: Some("Server session is Wayland.".into()),
                                }));
                                log::info!(
                                    "Rejected WebSocket input. Input isn't supported on Wayland.",
                                );
                            }
                        }
                    }
                    _ => log::error!("InputHandler: Invalid data {}", text),
                }
            }
            _ => (),
        }
    }
}

pub async fn handle(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let app_data: &Data<Config> = req.app_data().ok_or(ServerError::InternalError {
        message: "No app data".into(),
    })?;

    let config = app_data.lock().await;

    let pw = config.server.password.clone();

    std::mem::drop(config);

    ws::start(InputHandler { password: pw }, &req, stream)
}

fn get_data_type(data: &str) -> i32 {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.is_empty() {
        -1
    } else {
        parts[0].parse::<i32>().unwrap()
    }
}

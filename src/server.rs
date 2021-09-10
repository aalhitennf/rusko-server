use actix_web::{
    guard,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{input, middleware, routes, Config, Result};

pub async fn start(config: Config) -> Result<()> {
    let address = config.lock().await.get_address();

    HttpServer::new(move || {
        let auth_dev = HttpAuthentication::bearer(middleware::auth);
        let auth_jwt = HttpAuthentication::bearer(middleware::jwt);

        App::new()
            .app_data::<Data<Config>>(Data::new(config.clone()))
            .wrap(Logger::new("%t %r from %a -> %s in %D ms"))
            .service(
                web::scope("/api")
                    .wrap(auth_dev)
                    .service(
                        web::resource("/run")
                            .guard(guard::Post())
                            .route(web::post().to(routes::run)),
                    )
                    .service(
                        web::resource("/commands")
                            .guard(guard::Get())
                            .route(web::get().to(routes::commands)),
                    )
                    .service(
                        web::resource("/status")
                            .guard(guard::Get())
                            .route(web::get().to(routes::status)),
                    )
                    .service(
                        web::resource("/unpair")
                            .guard(guard::Post())
                            .route(web::route().to(routes::unpair)),
                    )
                    .service(
                        web::resource("/upload")
                            .guard(guard::Post())
                            .route(web::route().to(routes::upload)),
                    )
                    .route("/ws", web::get().to(input::handler::handle)),
            )
            .service(
                web::resource("/pair")
                    .wrap(auth_jwt)
                    .guard(guard::Post())
                    .route(web::route().to(routes::pair)),
            )
    })
    .bind(address)?
    .run()
    .await
    .map_err(Into::into)
}

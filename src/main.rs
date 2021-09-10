use std::sync::Arc;

use tokio::sync::Mutex;

mod config;
mod errors;
mod input;
mod middleware;
mod routes;
mod server;
mod test;
mod utils;

use utils::dirs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Config = Arc<Mutex<config::Config>>;

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = utils::args::get();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(args.log));

    let config_paths = dirs::directories(None)?;

    let config: Config = Arc::new(Mutex::new(config::Config::new(config_paths)?));

    config::monitor::start(config.clone()).await;

    #[cfg(target_os = "linux")]
    utils::signals::register()?;

    server::start(config).await
}

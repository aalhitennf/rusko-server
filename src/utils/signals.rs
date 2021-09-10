use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGQUIT},
    iterator::Signals,
};

use crate::Result;

pub fn register() -> Result<()> {
    let mut signals = Signals::new(&[SIGHUP, SIGINT, SIGQUIT])?;
    std::thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGINT || sig == SIGHUP || sig == SIGQUIT {
                std::process::exit(0);
            };
        }
    });
    Ok(())
}

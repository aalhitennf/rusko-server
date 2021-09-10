use clap::{App, Arg};

pub struct Args {
    pub log: &'static str,
}

pub fn get() -> Args {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("log")
                .short("v")
                .long("verbose")
                .help("Enable console logging."),
        )
        .get_matches();

    Args {
        log: if args.is_present("log") {
            "info"
        } else {
            "off"
        },
    }
}

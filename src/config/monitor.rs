// Thanks to Alacritty
use std::{sync::mpsc::channel, time::Duration};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use crate::Config;

pub async fn start(config: Config) {
    let tc = config.lock().await;
    let dirs = tc.dirs.clone();
    std::mem::drop(tc);

    let files_to_watch = vec![dirs.commands_file.clone(), dirs.paired_devices_file.clone()];

    actix_rt::spawn(async move {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        for file in files_to_watch {
            watcher.watch(file, RecursiveMode::NonRecursive).unwrap();
        }

        loop {
            let event = match rx.recv() {
                Ok(event) => event,
                Err(e) => {
                    log::error!("Monitoring failed: {}", e);
                    break;
                }
            };

            match event {
                DebouncedEvent::Write(path) => {
                    if path.eq(&dirs.commands_file) {
                        if config.lock().await.update_commands().is_ok() {
                            log::info!("Commands updated");
                        } else {
                            log::error!("Failed to update commands.");
                        }
                    }

                    if path.eq(&dirs.paired_devices_file) {
                        if config.lock().await.update_paired_devices().is_ok() {
                            log::info!("Paired devices updated");
                        } else {
                            log::error!("Failed to update paired devices.");
                        }
                    }

                    continue;
                }
                _ => continue,
            }
        }
    });
    log::info!("Monitoring OK");
}

// TODO Tests

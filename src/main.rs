use std::path::PathBuf;
use std::sync::mpsc::channel;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use clap::Parser;
use log::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long = "watch-for")]
    watch_for: PathBuf,
}

fn wait_for_file(path: &PathBuf) -> notify::Result<()> {
    let (tx, rx) = channel();

    let parent = path.parent().ok_or_else(|| {
        notify::Error::generic(&format!("Invalid path: {:?}", path))
    })?;

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            tx.send(res).unwrap();
        },
        notify::Config::default(),
    )?;
    watcher.watch(parent, RecursiveMode::NonRecursive)?;

    let target = std::fs::canonicalize(path).unwrap_or_else(|_| path.clone());

    info!("Waiting for {}", target.display());

    if target.exists() {
        info!("Trigger received, continue ...");
        return Ok(());
    }

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    // Only react to Create events
                    if let notify::event::EventKind::Create(_) = event.kind {
                        for p in event.paths {
                            let event_path = std::fs::canonicalize(&p).unwrap_or(p.clone());
                            if event_path == target {
                                info!("Trigger received, continue ...");
                                return Ok(());
                            }
                        }
                    }
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(notify::Error::generic(&format!("Watch error: {e}"))),
            }
        }

}

fn main() {
    // Always show info logs if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
    let args = Args::parse();

    match wait_for_file(&args.watch_for) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            error!("Error: {e}");
            std::process::exit(1);
        }
    }
}
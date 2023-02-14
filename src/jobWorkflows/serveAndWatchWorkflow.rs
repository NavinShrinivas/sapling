use crate::{serveSite, CustomError, RenderEnv};
use log::info;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Instant;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tower_livereload::Reloader;
pub async fn serve(local_render_env: &'static RenderEnv) -> Result<(), CustomError> {
    ctrlc::set_handler(move || {
        info!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let (rx, mut tx): (Sender<Reloader>, Receiver<Reloader>) = mpsc::channel(2);
    let server = serveSite::ServeSite::toweraxum_server(local_render_env, rx);
    tokio::spawn(async {
        if local_render_env.serve {
            let _ = &server.await;
        } else {
            return;
        }
    });
    let reload_handle = tx.recv().await.unwrap();
    tokio::spawn(async {
        change_detector(reload_handle, local_render_env).await;
    })
    .await
    .unwrap();
    Ok(())
}

pub async fn change_detector(reload_handle: Reloader, local_render_env: &'static RenderEnv) {
    // let mut watcher = RecommendedWatcher::new(
    //     move |_| {
    //         let start = Instant::now();
    //         info!("Change detected, reloading all sessions!");
    //         super::renderWorkflow::renderJob(local_render_env).unwrap();
    //         innerreload.reload();
    //         let duration = start.elapsed();
    //         info!("Rerender and reloading success!");
    //         info!("Reloading took : {:?}", duration);
    //     },
    //     Config::default(),
    // );

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default());
    watcher
        .as_mut()
        .unwrap()
        .watch(Path::new("./content/"), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .as_mut()
        .unwrap()
        .watch(Path::new("./templates/"), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .as_mut()
        .unwrap()
        .watch(Path::new("./css/"), RecursiveMode::Recursive)
        .unwrap();
    for res in rx {
        match res {
            Ok(_) => {
                let start = Instant::now();
                info!("Change detected, reloading all sessions!");
                super::renderWorkflow::renderJob(local_render_env).unwrap();
                reload_handle.reload();
                let duration = start.elapsed();
                info!("Rerender and reloading success!");
                info!("Reloading took : {:?}", duration);
            }
            _ => continue,
        }
    }
}

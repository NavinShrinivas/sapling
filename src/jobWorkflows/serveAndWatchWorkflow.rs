use crate::{serveSite, CustomError, RenderEnv, jobWorkflows::renderWorkflow};
use log::info;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Instant;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tower_livereload::Reloader;
pub async fn serve(local_render_env: &'static RenderEnv, settings : &'static std::collections::HashMap<String, serde_yaml::value::Value>) -> Result<(), CustomError> {
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
        change_detector(reload_handle, local_render_env, settings).await;
    })
    .await
    .unwrap();
    Ok(())
}

pub async fn change_detector(reload_handle: Reloader, local_render_env: &'static RenderEnv, settings : &'static std::collections::HashMap<String, serde_yaml::value::Value>) {
    let reloadarc = Arc::new(reload_handle);
    loop {
        let innerreload = Arc::clone(&reloadarc);
        let mut watcher = RecommendedWatcher::new(
            move |_| {
                let start = Instant::now();
                info!("Change detected, reloading all sessions!");
                //May not work at the moment, live reload broken, due to lack to async closures
                tokio::spawn(async{
                    renderWorkflow::parallel_renderJob(local_render_env, settings).await.unwrap();
                });
                innerreload.reload();
                let duration = start.elapsed();
                info!("Rerender and reloading success!");
                info!("Reloading took : {:?}", duration);
            },
            Config::default(),
        );

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
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
    }
}

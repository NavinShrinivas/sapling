use crate::{serveSite, CustomError, RenderEnv};
use rocket::Shutdown;

pub async fn serve(local_render_env: &'static RenderEnv) -> Result<(), CustomError> {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let server_handle = serveSite::ServeSite::actix_serve(local_render_env);
    tokio::spawn(async {
        if local_render_env.serve {
            let _ = &server_handle.await;
        }
    }).await.unwrap();
    loop {}
    Ok(())
}

pub async fn change_detector(server_shutdown_handle: Shutdown, local_render_env: RenderEnv) {
    println!("JHello");
    server_shutdown_handle.await
}

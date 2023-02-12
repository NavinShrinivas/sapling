use axum::http;
use axum::Router;
use std::path::Path;
use tokio::sync::mpsc::Sender;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
#[allow(dead_code)]
#[allow(non_snake_case)]
pub async fn toweraxum_server(
    local_render_env: &'static crate::RenderEnv,
    sendereloader: Sender<Reloader>,
) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service(
            "/",
            axum::routing::get_service(ServeDir::new(Path::new("./static/"))).handle_error(
                |e| async move {
                    (
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", e),
                    )
                },
            ),
        )
        .layer(TraceLayer::new_for_http())
        .layer(livereload);
    sendereloader.send(reloader).await.unwrap();
    let address = format!("{}:{}","0.0.0.0", String::from(local_render_env.serve_port.clone()));
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

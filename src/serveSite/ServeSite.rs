use axum::body::boxed;
use axum::body::Body;
use axum::body::BoxBody;
use axum::http::Request;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::Response;
use axum::routing::method_routing::get;
use axum::Router;
use log::error;
use log::warn;
use tokio::sync::mpsc::Sender;
use tower::util::ServiceExt;
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
    let livereload: LiveReloadLayer = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app: Router;
    if local_render_env.livereload {
        warn!("Avoid using firefox while using livereload");
        app = Router::new()
            .nest_service("/", get(handler))
            .layer(TraceLayer::new_for_http())
            .layer(livereload);
        sendereloader.send(reloader).await.unwrap();
    } else {
        app = Router::new()
            .nest_service("/", get(handler))
            .layer(TraceLayer::new_for_http());
    }
    let address = format!(
        "{}:{}",
        "0.0.0.0",
        String::from(local_render_env.serve_port.clone())
    );
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = path_includer(uri.clone()).await.unwrap();
    if res.status().is_success() == false {
        match format!("{}/index.html", uri).parse() {
            Ok(new_uri) => path_includer(new_uri).await,
            Err(_) => {
                error!("Error parsing in handler service");
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URL".to_string()))
            }
        }
    } else {
        Ok(res)
    }
}

async fn path_includer(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    match ServeDir::new("./static/").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(_) => {
            error!("Error inside file finder service");
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Error")))
        }
    }
}

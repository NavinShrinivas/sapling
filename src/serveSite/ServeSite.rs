use actix_files as fs;
use actix_web::middleware::Logger;
#[allow(dead_code)]
#[allow(non_snake_case)]
use actix_web::{App, HttpServer};

//Secondary server
pub fn rocket_serve(local_render_env: &crate::RenderEnv) -> rocket::Rocket<rocket::Build> {
    //To satisfy runtime serves :
    let mut static_path = std::env::current_dir().unwrap();
    static_path.push("static");
    let cfg = rocket::Config {
        port: local_render_env.serve_port.parse().unwrap(),
        address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        ..Default::default()
    };
    println!("Serving site on port : {:?}", cfg.port);
    rocket::build()
        .configure(cfg)
        .mount("/", rocket::fs::FileServer::from(static_path))
}

pub async fn actix_serve(local_render_env: &'static crate::RenderEnv){
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind(("0.0.0.0", local_render_env.serve_port.parse().unwrap()))
    .unwrap().run();
    server.await.unwrap();
    // return server.handle()
}

#[allow(dead_code)]
#[allow(non_snake_case)]
use rocket;
// use std::rc::Rc;
// use crate::RenderEnv;

pub fn rocket_serve(local_render_env : &crate::RenderEnv) -> rocket::Rocket<rocket::Build> {
    //To satisfy runtime serves :
    let mut static_path = std::env::current_dir().unwrap();
    static_path.push("static");
    let cfg = rocket::Config{
        port : local_render_env.serve_port.parse().unwrap(),
        address : std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0)),
        ..Default::default()
    };
    println!("Serving site on port : {:?}",cfg.port);
    rocket::build().configure(cfg).mount("/", rocket::fs::FileServer::from(static_path))
}

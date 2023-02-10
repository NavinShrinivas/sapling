#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
//Crate module tree additions : mod bootstrap;
mod bootstrap;
mod jobWorkflows;
mod loadMemory;
mod parseMarkdown;
mod parseTemplate;
mod renderMarkdown;
mod serveSite;

//External crates
use clap::{Parser, Subcommand};
use env_logger::Env;
use tokio;

//[PENDING] Refactor to workflows
use bootstrap::Bootstrap;

#[derive(Debug)]
pub enum CustomErrorStage {
    ParseTemplates,
    ParseMarkdown,
    StaticRender,
    Bootstrap,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct CustomError {
    stage: CustomErrorStage,
    error: String,
}

#[derive(Parser)]
#[command(name = "sapling")]
#[command(version = "1.0")]
#[command(about = "Static site generator", long_about = None)]
pub struct RenderEnv {
    #[arg(long, default_value = "templates")]
    template_base: String,
    #[arg(long, default_value = "content")]
    content_base: String,
    #[arg(long, default_value = "static")]
    static_base: String,
    #[arg(long, default_value = "css")]
    css_base: String,
    #[arg(long, default_value = "assets")]
    assets_base: String,
    #[arg(long, default_value = "index.html")]
    default_template: String,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    serve: bool,
    #[arg(long, default_value = "80")]
    serve_port: String,
    #[command(subcommand)]
    mode: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// To create a new project. To create project in current folder use path as `.`
    Bootstrap {
        project_name: Option<String>,
    },
    Run,
}

#[tokio::main]
async fn main() {
    static local_render_env: once_cell::sync::Lazy<RenderEnv> =
        once_cell::sync::Lazy::new(|| RenderEnv::parse());
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    match &local_render_env.mode {
        Commands::Bootstrap { project_name } => match project_name {
            Some(name) => match Bootstrap::bootstrapper(name.to_string()) {
                Ok(_) => {}
                Err(e) => {
                    println!("ran into error while bootstrapping a new project");
                    panic!("{:?}", e);
                }
            },
            None => {
                panic!("[ERROR] No project name or path given to create new project.")
            }
        },
        _ => {
            jobWorkflows::renderWorkflow::renderJob(&local_render_env).unwrap();
            jobWorkflows::serveAndWatchWorkflow::serve(&local_render_env)
                .await
                .unwrap();
            println!(); //Just to flush
        }
    }
}
// server_handle.await

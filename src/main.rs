#![allow(dead_code)]
#![allow(non_snake_case)]
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tokio;

mod bootstrap;
mod jobWorkflows;
mod loadMemory;
mod parseMarkdown;
mod parseTemplate;
mod renderMarkdown;
mod serveSite;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discovered {
    //File path is the key and document matter is the value
    data: std::collections::HashMap<String, parseMarkdown::ParseMarkdown::ContentDocument>,
}

impl Default for Discovered {
    fn default() -> Self {
        return Discovered {
            data: std::collections::HashMap::new(),
        };
    }
}

#[derive(Subcommand)]
enum Commands {
    /// To create a new project. To create project in current folder use path as `.`
    Bootstrap {
        project_name: Option<String>,
    },
    Run,
}

impl Default for RenderEnv {
    fn default() -> Self {
        Self {
            template_base: "templates".to_string(),
            content_base: "content".to_string(),
            static_base: "static".to_string(),
            css_base: "css".to_string(),
            assets_base: "assets".to_string(),
            default_template: "index.html".to_string(),
            serve_port: "80".to_string(),
            debug: false,
            serve: true,
            mode: Commands::Bootstrap { project_name: None },
        }
    }
}

#[tokio::main]
async fn main() {
    let local_render_env = RenderEnv::parse();

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
            let server = async {
                if local_render_env.serve == true {
                    match serveSite::ServeSite::rocket_serve(&local_render_env)
                        .launch()
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("[ERROR] serving static files failed : {}", e)
                        }
                    };
                }
            };
            server.await;
            println!(); //Just to flush
        }
    }
}

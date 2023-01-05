#[allow(dead_code)]
#[allow(non_snake_case)]
use std::rc::Rc;
use tokio;

mod markdown_parser;
mod parse_templates;
mod render_markdown;
mod serve_site;

use parse_templates::ParseTemplates;
use parse_templates::ParseTemplates::TemplatesMetaData;
use render_markdown::RenderMarkdown;

#[derive(Debug)]
pub enum CustomErrorStage {
    ParseTemplates,
    ParseMarkdown,
    StaticRender,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct CustomError {
    stage: CustomErrorStage,
    error: String,
}

pub struct RenderEnv {
    template_base: String,
    content_base: String,
    static_base: String,
    css_base: String,
    default_template: String,
    debug: bool,
}

impl Default for RenderEnv {
    fn default() -> Self {
        Self {
            template_base: "templates".to_string(),
            content_base: "content".to_string(),
            static_base: "static".to_string(),
            css_base: "css".to_string(),
            default_template: "index.html".to_string(),
            debug: true,
        }
    }
}
impl RenderEnv {
    #[allow(dead_code)]
    fn new<S: std::string::ToString>(
        template_base: S,
        content_base: S,
        static_base: S,
        css_base: S,
        default_template: S,
    ) -> RenderEnv {
        RenderEnv {
            template_base: template_base.to_string(),
            content_base: content_base.to_string(),
            static_base: static_base.to_string(),
            css_base: css_base.to_string(),
            default_template: default_template.to_string(),
            debug: false,
        }
    }
}

#[tokio::main]
async fn main() {
    //[TODO]convert the below params to command line args
    let global_render_env = Rc::default();
    let local_render_env = Rc::clone(&global_render_env);

    let template_meta = match ParseTemplates::TemplatesMetaData::new(local_render_env) {
        Ok(s) => {
            println!("All detected templates parsed without errors!");
            s
        }
        Err(e) => {
            println!("Ran into error while parsing templates.");
            panic!("{}", e)
        }
    };

    match RenderMarkdown::static_render(Rc::clone(&global_render_env), &template_meta) {
        Ok(_) => {
            println!("All markdown content rendered without errrors!")
        }
        Err(e) => {
            println!("Ran into error while rendering markdown.");
            panic!("{:?}", e)
        }
    }
    match serve_site::ServeSite::rocket_serve().launch().await {
        Ok(_) => {}
        Err(e) => {
            panic!("[ERROR] serving static files failed : {}", e)
        }
    };
}

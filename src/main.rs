#[allow(dead_code)]
#[allow(non_snake_case)]

use std::rc::Rc;
use tokio;

mod markdown_parser;
mod render_markdown;
mod parse_templates; 
mod serve_site;

use parse_templates::ParseTemplates;
use parse_templates::ParseTemplates::TemplatesMetaData;
use render_markdown::RenderMarkdown;

#[derive(Debug)]
pub enum CustomErrorType {
    StaticRender,
}
#[derive(Debug)]
pub struct CustomError {
    r#type: CustomErrorType,
    error: String,
}

pub struct RenderEnv {
    template_base: String,
    content_base: String,
    static_base: String,
    css_base: String,
    default_template: String,
}
impl RenderEnv {
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
        }
    }
}

#[tokio::main]
async fn main() {
    //[TODO]convert the below params to command line args
    let global_render_env = Rc::new(RenderEnv::new(
        "templates",
        "content",
        "static",
        "css",
        "index.html",
    ));
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
    serve_site::ServeSite::rocket_serve().launch().await.unwrap();
}

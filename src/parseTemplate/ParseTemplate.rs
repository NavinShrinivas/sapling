use crate::RenderEnv;
use log::info;
#[allow(dead_code)]
#[allow(non_snake_case)]
use tera::Tera;

#[derive(Debug)]
pub struct TemplatesMetaData {
    pub compiled_tera_instance: Tera,
}

impl TemplatesMetaData {
    ///Renders all tempaltes and supports inheritance between template as we are usine Tera blobs
    pub fn new(local_render_env: &RenderEnv) -> Result<TemplatesMetaData, tera::Error> {
        // [TODO] To have stylised components, we must bundle and minify all the css to one file
        // Problem being that if we have two classes with same name it will clash

        // As for now we can just get all the styles in one place and move that to the rendered
        // directory - Done in RenderMarkdown.rs

        let tera_instance = match Tera::new(&format!("{}/**/*", local_render_env.template_base)) {
            Ok(tera) => tera,
            Err(e) => return Err(e),
        };
        if local_render_env.debug {
            info!(
                "{:#?}",
                tera_instance.get_template_names().collect::<Vec<_>>()
            );
        }

        Ok(TemplatesMetaData {
            compiled_tera_instance: tera_instance,
        })
    }
}

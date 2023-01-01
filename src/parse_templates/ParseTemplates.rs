#[allow(dead_code)]
#[allow(non_snake_case)]

use tera::Tera;
use crate::RenderEnv;
use std::rc::Rc;

#[derive(Debug)]
pub struct TemplatesMetaData {
    pub compiled_tera_instance: Tera,
}
impl TemplatesMetaData {
    pub fn new(local_render_env: Rc<RenderEnv>) -> Result<TemplatesMetaData, tera::Error> {
        //This is because, Tera can do inherited templates only if we use its blob function
        let tera_instance = match Tera::new(&format!("{}/**/*", local_render_env.template_base)) {
            Ok(tera) => tera,
            Err(e) => return Err(e),
        };
        println!("Templates detected : ");
        println!(
            "{:#?}",
            tera_instance.get_template_names().collect::<Vec<_>>()
        );

        Ok(TemplatesMetaData {
            compiled_tera_instance: tera_instance,
        })
    }
}


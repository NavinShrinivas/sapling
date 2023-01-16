use crate::RenderEnv;
#[allow(dead_code)]
#[allow(non_snake_case)]
use tera::Tera;

#[derive(Debug)]
pub struct TemplatesMetaData {
    pub compiled_tera_instance: Tera,
}

impl TemplatesMetaData {

    //Renders all tempaltes and supports inheritance between template as we are usine Tera blobs
    pub fn new(local_render_env: &RenderEnv) -> Result<TemplatesMetaData, tera::Error> {
        //[TODO]
        // To have stylised components, we must bundle and minify all the css to one file
        // Problem being that if we have two classes with same name it will clash

        //As for now we can just get all the styles in one place and move that to the rendered
        //directory - Done in RenderMarkdown.rs

        let tera_instance = match Tera::new(&format!("{}/**/*", local_render_env.template_base)) {
            Ok(tera) => tera,
            Err(e) => return Err(e),
        };
        if local_render_env.debug {
            println!("[DEBUG] Templates detected : ");
            println!(
                "{:#?}",
                tera_instance.get_template_names().collect::<Vec<_>>()
            );
        }

        Ok(TemplatesMetaData {
            compiled_tera_instance: tera_instance,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::RenderEnv;

    fn test_setup(){
        std::fs::create_dir_all("test/templates").unwrap();
        let test_template = "
<html>
   <head>
      {% block head %}
      <title>{{ frontmatter.title.main }}</title>
      <link rel='stylesheet' href='/css/index.css' />
      {% endblock head %}
   </head>
   <body class='hello'>
      {% block content %}
      {# Children can override this block, unless they use the variable super() in which case its added on #}
      {{ content|safe }}
      {% endblock content %}
   </body>
</html>
        ";
        std::fs::write("test/templates/index.html", test_template).unwrap();
    }

    #[test]
    fn test_basic_template_parsing() {
        test_setup();
        let test_render_env = RenderEnv {
            template_base: "test/templates".to_string(),
            ..Default::default()
        };
        super::TemplatesMetaData::new(&test_render_env).unwrap();
        test_cleanup();
    }

    fn test_cleanup(){
        std::fs::remove_dir_all("test").unwrap();
    }
}

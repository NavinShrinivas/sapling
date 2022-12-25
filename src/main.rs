use tera::Tera;
use walkdir::WalkDir;

#[derive(Debug)]
struct TemplatesMetaData {
    relative_paths: Vec<String>, //Paths of templates relative to `template_base` folder
}
impl TemplatesMetaData {
    fn new() -> TemplatesMetaData {
        TemplatesMetaData {
            relative_paths: Vec::new(),
        }
    }
}

fn discover_templates<S: std::string::ToString + AsRef<std::path::Path>>(
    template_meta: &mut TemplatesMetaData,
    template_base: S,
) -> Result<String, walkdir::Error> {
    for i in WalkDir::new(template_base) {
        let entry = match i {
            Ok(i) => {
                if i.file_type().is_dir(){
                    continue;
                }else{
                    i
                }
            },
            Err(e) => return Err(e),
        };
        template_meta
            .relative_paths
            .push(entry.path().display().to_string());
    }
    Ok(String::from(
        "Successfully discovered all templates from given base dir.",
    ))
}

fn main() {
    //Not using blobs for rendering templates in Tera, using walkdir instead
    let template_base = "./templates"; //[TODO] make this a command line arg
    let mut template_meta = TemplatesMetaData::new();
    match discover_templates(&mut template_meta, template_base) {
        Ok(res) => {
            println!("{}", res);
            println!("{:#?}",template_meta);
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }
}


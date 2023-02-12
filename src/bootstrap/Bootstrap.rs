use std::fs;
use crate::{CustomError,CustomErrorStage};
use walkdir::WalkDir;
use log::{ info };
pub fn bootstrapper(project_name : String) -> Result<(),CustomError>{
    match fs::read_dir(&project_name){
        Ok(mut dir) => {
            match &dir.next(){
                Some(_) => {
                    return Err(CustomError{
                        stage : CustomErrorStage::Bootstrap,
                        error : format!("[ERROR] Folder is not empty!")
                    })
                }
                None => {}
            }
        },
        Err(_) => {
            info!("Folder not found, creating one!");
            match fs::create_dir(&project_name){
                Ok(_) => {},
                Err(e) => {
                    return Err(CustomError{
                        stage : CustomErrorStage::Bootstrap,
                        error : format!("[ERROR] Couldnt create folder for new project : {}",e)
                    })
                }
            }
        }
    };
    match copy_bootstrapper_project(project_name){
        Ok(()) => {},
        Err(e) => {
            return Err(e)
        }
    }
    Ok(())
}
fn copy_bootstrapper_project(project_name : String) -> Result<(),CustomError>{
    let root = "/usr/local/src/sapling/bootstrap_project";
    let content_walker = WalkDir::new("/usr/local/src/sapling/bootstrap_project");
    for i in content_walker.into_iter() {
        let entry = match i {
            Ok(entry) => {entry},
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::StaticRender,
                    error: format!("[ERROR] Dir entry error : {}", e),
                })
            }
        };
        let path = entry.path();
        let path_str = path.strip_prefix(&root).unwrap().to_str().unwrap().trim_start_matches("/");
        let name = project_name.trim_end_matches("/");
        let project_path = format!("{}/{}",name,path_str);
        info!("{} {}",path.display(), project_path);
        if path.is_file() {
            std::fs::copy(path,project_path).unwrap();
        }
        else{
            std::fs::create_dir_all(project_path).unwrap();
        }
    }
    Ok(())
}

use std::fs;
use crate::{CustomError,CustomErrorStage};

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
            println!("[INFO] Folder not found, creating one!");
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
    create_folders(&project_name);
    create_basic_project(&project_name);
    Ok(())
}

fn create_folders(project_name : &String){
    let name = project_name.trim_end_matches("/");
    fs::create_dir_all(format!("{}/{}",name,"content")).unwrap();
    fs::create_dir_all(format!("{}/{}",name,"templates")).unwrap();
    fs::create_dir_all(format!("{}/{}",name,"css")).unwrap();
}

fn create_basic_project(project_name : &String){
    let name = project_name.trim_end_matches("/");
}

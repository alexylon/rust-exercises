use substring::Substring;
use std::fs;
use std::error::Error;
use walkdir::WalkDir;

pub fn get_substring() -> Result<(), Box<dyn Error>> {
    let str = ".vpms/modules/Life/0.0.0/pms/AdminLoans.pms";
    let first_index = str.find('/').unwrap();
    let last_index = str.rfind('/').unwrap();
    let dir_path = str.substring(first_index + 1, last_index + 1);
    let file_path = str.substring(first_index + 1, str.len());
    let first_index_module = file_path.find('/').unwrap();
    let module= file_path.substring(0, first_index_module);
    let _test: String = str.chars().skip(first_index + 1).take(2).collect();
    let filename = str.substring(last_index + 1, str.len());
    println!("The dir_path is {}", dir_path);
    println!("The file_path is {}", file_path);
    println!("The filename is {}", filename);
    println!("The module is {}", module);

    // let module_paths = fs::read_dir("modules")?;
    // for paths_1 in module_paths {
    //     for paths_2 in paths_1 {
    //         // if let dir_entry = paths_2 {
    //             let file_path = format!("{}",
    //                                     &paths_2.path().display());
    //             println!("module_path: {:#?}", file_path);
    //         // }
    //     }
    // }

    // for entry in WalkDir::new("modules").into_iter().filter_map(|e| e.ok()) {
    //     if let Some(path_str) = entry.path().to_str() {
    //         if let Some(last_dot) = path_str.rfind('.') {
    //             let extension = path_str.substring(last_dot + 1, path_str.len());
    //             if let Some(first_slash) = path_str.find('/') {
    //                 let module_path = path_str.substring(first_slash + 1, path_str.len());
    //                 if let Some(first_slash) = module_path.find('/') {
    //                     if let Some(last_slash) = module_path.rfind('/') {
    //                         if let Some(last_dot) = module_path.rfind('.') {
    //                             let module = module_path.substring(0, first_slash);
    //                             if extension == "pms" || extension == "vpm" {
    //                                 let name = module_path.substring(last_slash + 1, last_dot);
    //                                 println!("{}", entry.path().display());
    //                                 println!("module: {}", module);
    //                                 println!("name: {}", name);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    Ok(())
}
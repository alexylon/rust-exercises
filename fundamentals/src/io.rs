use std::error::Error;
use std::fs;

pub fn io_test1() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir("C:\\AllowedValues").unwrap();
    fs::create_dir_all(format!("{}/{}/", "test", "AllowedValues"))?;

    for path in paths {
        let file_path = format!("{}", &path.as_ref().unwrap().path().display());
        let filename = &path.unwrap().file_name().into_string().unwrap()[..];
        println!("{}", filename);
        fs::copy(&file_path, format!("{}/{}/{}", "test", "AllowedValues", &filename))?;
    }

    Ok(())
}
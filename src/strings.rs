use substring::Substring;

pub fn get_substring() {
    let str = ".vpms/modules/Life/0.0.0/pms/AdminLoans.pms";
    let first_index = str.find('/').unwrap();
    let last_index = str.rfind('/').unwrap();
    let dir_path = str.substring(first_index + 1, last_index + 1);
    let file_path = str.substring(first_index + 1, str.len());
    let _test: String = str.chars().skip(first_index + 1).take(2).collect();
    println!("The dir_path is {}", dir_path);
    println!("The file_path is {}", file_path);
}
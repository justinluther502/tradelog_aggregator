use glob::glob;
use std::path::PathBuf;
use std::env;

fn main() {
    let mut searchstr = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    searchstr.push_str("/*.csv");
    let file_list = list_csvs(&searchstr);
    println!("{:?}", file_list);
}

fn list_csvs(filepath: &str) -> Vec<PathBuf> {
    let mut csv_list = Vec::new();
    for entry in glob(filepath).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => csv_list.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    csv_list
}
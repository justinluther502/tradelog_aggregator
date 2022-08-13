use glob::glob;
use std::env;
use csv::{Reader, StringRecord, Writer};

fn main() {
    let mut searchstr = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    searchstr.push_str("/*.csv");
    let file_list = list_csvs(&searchstr);
    let rows = read_all_csvs(file_list);
    write_combined_csv(rows);
}

fn list_csvs(searchstr: &str) -> Vec<String> {
    let mut csv_list = Vec::new();
    for entry in glob(searchstr).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => csv_list.push(path.into_os_string().into_string().unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }
    csv_list
}

fn read_file(filepath: &str) -> Vec<StringRecord> {
    let mut rdr = Reader::from_path(filepath).unwrap();
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        rows.push(record);
    }
    rows
}

fn read_all_csvs(files: Vec<String>) -> Vec<StringRecord> {
    let mut rows = Vec::new();
    let mut rdr = Reader::from_path(&files[0]).unwrap();
    let headers = rdr.headers().unwrap().to_owned();
    rows.push(headers);

    for file in files {
        let mut new_rows = read_file(&file);
        rows.append(&mut new_rows);
    }
    rows    
}

fn write_combined_csv(rows: Vec<StringRecord>) {
    let mut wtr = Writer::from_path("consolidated.csv").unwrap();
    for row in rows {
        wtr.write_record(&row);
    }
    wtr.flush();
}
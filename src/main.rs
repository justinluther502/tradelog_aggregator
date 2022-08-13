use glob::glob;
// use std::env;
use std::{error::Error, str::FromStr};
use csv::{Reader, StringRecord, Writer};
use chrono;

fn main() {
    // let mut searchstr = env::current_dir()
    //     .unwrap()
    //     .into_os_string()
    //     .into_string()
    //     .unwrap();
    // searchstr.push_str("/*.csv");
    let searchstr = "C:\\Users\\justi\\Documents\\LWM\\7. Trade Logs/*.csv";
    let file_list = list_csvs(&searchstr);
    let rows = read_all_csvs(file_list);
    write_combined_csv(rows).unwrap();
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
    for record in rdr.records() {
        let row = record.unwrap();
        rows.push(row);
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

fn write_combined_csv(rows: Vec<StringRecord>) -> Result<(), Box<dyn Error>> {
    let mut filename = String::from_str("Consolidated Logs - ")?;
    filename.push_str(&chrono::offset::Local::now().to_string()[..10]);
    filename.push_str(".csv");
    let mut wtr = Writer::from_path(filename)?;
    for row in rows {
        wtr.write_record(&row)?;
    }
    wtr.flush()?;
    Ok(())
}
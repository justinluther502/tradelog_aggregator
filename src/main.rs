use chrono;
use csv::{Reader, StringRecord, Writer};
use glob::glob;
use std::env;
use std::{error::Error, path::PathBuf, str::FromStr};

fn main() {
    let mut searchpath = env::current_dir().unwrap();
    searchpath.push("*.csv");
    let file_list = list_csvs(searchpath);
    let rows = read_all_csvs(file_list);
    write_combined_csv(rows).unwrap();
}

fn list_csvs(searchpath: PathBuf) -> Vec<PathBuf> {
    let mut csv_list = Vec::new();
    let searchstr = searchpath.to_str().unwrap();
    for entry in glob(searchstr).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => csv_list.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    csv_list
}

fn read_file(filepath: PathBuf) -> Vec<StringRecord> {
    let mut rdr = Reader::from_path(filepath).unwrap();
    let mut rows = Vec::new();
    for record in rdr.records() {
        let row = record.unwrap();
        rows.push(row);
    }
    rows
}

fn read_all_csvs(files: Vec<PathBuf>) -> Vec<StringRecord> {
    let mut rows = Vec::new();
    let mut rdr = Reader::from_path(&files[0]).unwrap();
    let headers = rdr.headers().unwrap().to_owned();
    rows.push(headers);

    for file in files {
        let mut new_rows = read_file(file);
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

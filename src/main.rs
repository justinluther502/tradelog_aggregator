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
    // Lists all CSV files in the input search path.
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
    // Reads CSV at filepath and returns a vector of row records.
    let mut rdr = Reader::from_path(filepath).unwrap();
    let mut file_rows = Vec::new();

    for record in rdr.records() {
        let row = record.unwrap();
        file_rows.push(row);
    }
    file_rows
}

fn read_all_csvs(files: Vec<PathBuf>) -> Vec<StringRecord> {
    // Reads all CSVs in a vector and returns a vector of aggregated row records.
    let mut combined_rows = Vec::new();
    let mut rdr = Reader::from_path(&files[0]).unwrap();
    let headers = rdr.headers().unwrap().to_owned();
    combined_rows.push(headers);

    for file in files {
        let mut new_rows = read_file(file);
        combined_rows.append(&mut new_rows);
    }
    combined_rows
}

fn write_combined_csv(rows: Vec<StringRecord>) -> Result<(), Box<dyn Error>> {
    // Takes a vector of row records and writes a CSV with filename encoded by today's date.
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

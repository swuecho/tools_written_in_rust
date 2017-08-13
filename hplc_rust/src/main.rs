use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::fs::read_dir;
fn main() {
    // We assume that we are in a valid directory.
    let path = env::current_dir().unwrap();

    println!("The current directory is {}", path.display());
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match read_dir(&Path::new("./data")) {
        Err(why) => println!("! read dir {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let (name, data) = extract_data_from_file(path.unwrap().path().as_path());
                println!("{}contains:\n{:?}", name, data);
            }
        }
    }

}


fn extract_data_from_file(path: &Path) -> (String, HashMap<String, String>) {

    // let reader : BufReader<File> = BufReader::new(File::open(path).unwrap());
    let reader = BufReader::new(File::open(path).unwrap());
    // get the type of lines?

    let mut data_map = HashMap::new();
    let mut sample_name = String::new();

    let mut process_indicator = false;

    for line in reader.lines() {
        let line_content = line.unwrap();
        if line_content.contains("Sample Name") {
            sample_name = line_content.split("\t").nth(1).unwrap().to_string(); //map(ToOwned::to_owned);
        }

        if line_content.contains("Amount") {
            process_indicator = true;
        }

        if line_content.contains("Page") {
            process_indicator = false;
        }

        if process_indicator {
            let v: Vec<&str> = line_content.split("\t").collect();
            if v.len() == 6 {
                // data_map.insert(v[1].to_string(), v[5].to_string());
                data_map.insert(v[1].to_string(), v[5].to_string());
            }
        }
    }
    (sample_name, data_map)
}

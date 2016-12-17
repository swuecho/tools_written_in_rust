extern crate time;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::env;
fn main() {

    let path = env::home_dir().unwrap().join(".zsh_history");
    let reader = BufReader::new(File::open(path).unwrap());
    // get the type of lines?
    let mut data_map = HashMap::new();
    let current_time_unix = time::get_time().sec as i32;
    for line in reader.lines() {
        if line.is_ok() {
            let line_content = line.unwrap();
            let time_and_cmd: Vec<&str> = line_content.split(":").collect(); //.unwrap().map(to_string); //map(ToOwned::to_owned);
            if time_and_cmd.len() > 2 {
                let time = time_and_cmd[1];
                let cmd = time_and_cmd[2];
                let unix_time_parsed = (&time[1..]).parse::<i32>();
                if unix_time_parsed.is_ok() {
                    if current_time_unix - unix_time_parsed.unwrap() < 60 * 60 * 24 {
                        // println!("{}", &cmd[2..].to_string());
                        let cmd_clean = (&cmd[2..]).to_string();
                        *data_map.entry(cmd_clean).or_insert(0) += 1; // why * is nccessayr
                    }
                }
            }
        }
    }
    for (contact, &number) in data_map.iter() {
        println!("{}: {}", number, contact);
    }
}

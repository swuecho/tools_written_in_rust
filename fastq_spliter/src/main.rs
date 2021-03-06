extern crate bio;
extern crate rand;
use rand::thread_rng;
use bio::io::fastq;
use rand::Rng;
use std::env;

fn filter_records(in_file: &str, out_file: &str) {
    let reader = fastq::Reader::from_file(in_file).unwrap();
    let mut writer = fastq::Writer::to_file(out_file).unwrap();
    let mut rng = thread_rng();
    for result in reader.records() {
        let record = result.unwrap();
        if rng.gen() {
            writer.write_record(&record).unwrap();
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("error: 2 and only 2 args required");
        return;
    }
    let in_file = &args[0];
    let out_file = &args[1];
    filter_records(in_file, out_file)
}

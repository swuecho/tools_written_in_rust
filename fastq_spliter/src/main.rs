extern crate bio;
extern crate rand;
use rand::thread_rng;
use bio::io::fastq;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("error: 2 and only 2 args required");
        return;
    }
    let in_file = &args[0];
    let out_file = &args[1];
    let reader = fastq::Reader::from_file(in_file).unwrap();
    let mut writer = fastq::Writer::to_file(out_file).unwrap();
    let mut rng = thread_rng();
    // let records: Vec<io::Result<Record>> = reader.records().collect();
    for result in reader.records() {
        // obtain record or fail with error
        let record = result.unwrap();
        // obtain sequence
        // let seq = record.seq();
        // let s = String::from_utf8(seq).expect("Found invalid UTF-8");
        // random bool
        if rng.gen() {
            writer.write_record(&record).unwrap();
        }
    }
}

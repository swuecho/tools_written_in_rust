extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable)]
struct Student {
       id: u32,
       full_name: String,
       address : String,
       phone: String
}

#[derive(RustcDecodable)]
struct Instructor {
       id: u32,
       full_name: String,
       address : String,
       phone: String
}

fn get_students() ->std::result::Result<std::vec::Vec<Student>, csv::Error> {
        let mut rdr = csv::Reader::from_file("/home/hwu/dev/tools_written_in_rust/courses/src/data/students.csv").
            unwrap().has_headers(false);
        let rows = rdr.decode().collect::<csv::Result<Vec<Student>>>();
        return rows;

}

fn get_instructors() ->std::result::Result<std::vec::Vec<Instructor>, csv::Error> {
        let mut rdr = csv::Reader::from_file("/home/hwu/dev/tools_written_in_rust/courses/src/data/instructors.csv").
            unwrap().has_headers(false);
        let rows = rdr.decode().collect::<csv::Result<Vec<Instructor>>>();
        return rows;

}


fn main() {
        let rows = get_students();
        let records = rows.unwrap();
        let length = records.len();
        println!("{}", length);
        let ref record = records[0];
        println!("{}, {}, {},{}", record.id, record.full_name, record.address, record.phone);
        let instructors = get_instructors();
        let length = instructors.unwrap().len();
        println!("{}", length)
        //println!("{}, {}, {},{}", record.id, record.full_name, record.address, record.phone);
}

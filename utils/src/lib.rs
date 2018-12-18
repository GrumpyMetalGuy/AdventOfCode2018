use std::{
    io,
    io::{BufRead, BufReader, Read},
    fs::File,
};


pub fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


pub fn read_file(filename: String) -> String {
    let mut output = String::new();

    File::open(filename).expect("Unable to open file").read_to_string(&mut output).expect("Unable to read filename");

    output
}

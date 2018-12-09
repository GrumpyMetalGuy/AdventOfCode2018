use std::{
    io,
    io::{BufRead, BufReader},
    fs::File,
};


pub fn lines_from_file(filename: String) -> io::Result<Vec<String>>
{
    BufReader::new(File::open(filename)?).lines().collect()
}

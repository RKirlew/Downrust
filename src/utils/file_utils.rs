use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<T>(filePath:T)->io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filePath)?;
    Ok(io::BufReader::new(file).lines())
}
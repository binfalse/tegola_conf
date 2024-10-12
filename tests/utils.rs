use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};

fn read_lines(file: File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn compare_files(file1: File, file2: File) -> Result<bool, Box<dyn Error>> {
    let mut contents1: Vec<String> = read_lines(file1)
        .into_iter()
        .filter(|l| l.len() > 0 && !l.starts_with('#'))
        .collect();
    let mut contents2: Vec<String> = read_lines(file2)
        .into_iter()
        .filter(|l| l.len() > 0 && !l.starts_with('#'))
        .collect();

    contents1.sort();
    contents2.sort();

    if contents1.len() != contents2.len() {
        return Ok(false);
    }

    let mismatches = contents1
        .iter()
        .zip(contents2.iter())
        .filter(|&(a, b)| a != b)
        .count();

    Ok(mismatches == 0)
}

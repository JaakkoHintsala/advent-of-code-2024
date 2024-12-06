use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;

use regex::Regex;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root dir: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-3-second/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let mul_pairs = match parse_to_mul_pairs(delete_donts(input.clone())) {
        Ok(muls) => muls,
        Err(error) => {
            eprintln!("Error while parsing: {}", error);
            std::process::exit(1);
        }
    };

    let res: i64 = mul_pairs.iter().map(|(first, second)| first * second).sum();
    println!("{}", res);

}

fn delete_donts(input: String) -> String {

    let reg = Regex::new(r"(don't\(\)[\s\S]*?do\(\))").unwrap();
    let mut ret = input.clone();
    
    for (_, [dont_str]) in reg.captures_iter(&input).map(|c| c.extract()) {
        ret = ret.replace(dont_str, "");
    }

    ret
}

fn parse_to_mul_pairs(input: String) -> Result<Vec<(i64, i64)>, ParseIntError> {

    let reg = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)").unwrap();
    let mut muls = vec![];

    for (_, [first, second]) in reg.captures_iter(&input).map(|c| c.extract()) {
        muls.push((first.parse::<i64>()?, second.parse::<i64>()?));
    }
    Ok(muls)
}

fn read_and_process_input(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut input: String = String::new();
    reader.read_to_string(&mut input)?;
    Ok(input)
}

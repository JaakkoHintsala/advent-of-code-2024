use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

use regex::{Regex, Replacer};

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root dir: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-4-first/input.txt"));

    // print!("{}", input_path_str);

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let (hor_rows, ver_rows, diag1_rows, diag2_rows) = match get_orientations(input) {
        Ok(tuple) => tuple,
        Err(error) => {
            eprintln!("Error while parsing input rows: {}", error);
            std::process::exit(1);
        }
    };
    // println!("hor_rows: {:?}", hor_rows);
    // println!("ver_rows: {:?}", ver_rows);
    // println!("diag1_rows: {:?}", diag1_rows);
    // println!("diag2_rows: {:?}", diag2_rows);

    // seperate regexes are necessary because find_iter only finds non-overlapping matches
    let reg1 = Regex::new(r"(XMAS)").unwrap();
    let reg2 = Regex::new(r"(SAMX)").unwrap();

    let match_count: usize = hor_rows
        .iter()
        .chain(ver_rows.iter())
        .chain(diag1_rows.iter())
        .chain(diag2_rows.iter())
        .map(|row| reg1.find_iter(&row).count() + reg2.find_iter(&row).count())
        .sum();
    println!("{}", match_count);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Orientation {
    Vertical,
    Diagonal1,
    Diagonal2,
}

fn get_orientations(
    input: Vec<String>,
) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let input_row_length = match input
        .first()
        .map(|first_str| {
            first_str.len()
        })
        .filter(|len| *len > 0)
    {
        Some(length) => length,
        None => {
            let error: Box<dyn std::error::Error> = "Input cannot be empty".into();
            return Err(error);
        }
    };

  //  let len_as_i64 = i64::try_from(input_row_length)?;

    // let vert = vec![];
    // let diag = vec![];
    // let diag2 = vec![];

    let mut map: HashMap<(Orientation, i64), String> = HashMap::new();

    for char_index in 0..input_row_length {
        // &input.iter().enumerate().for_each(|(ind, row)| {
        // let c = match row.chars().nth(i) {
        //     Some(character) => character,
        //     None => {
        //         let error: Box<dyn std::error::Error> = ("input row ".to_string() + &(ind + 1).to_string() + " was not the same length as first row").into();
        //         return Err(error);
        //     },
        // };

        // let c = row.chars().nth(i).ok_or_else(|| {
        //     let error: Box<dyn std::error::Error> = ("input row ".to_string()
        //         + &(ind + 1).to_string()
        //         + " was not the same length as first row")
        //         .into();
        //     return Err(error);
        // })?;

        // let i_as_i64 = i64::try_from(input_row_length)?;
        // });

        for (row_index, row) in &input.iter().enumerate().collect::<Vec<_>>() {
            let c = match row.chars().nth(char_index) {
                Some(character) => character,
                None => {
                    let error: Box<dyn std::error::Error> = ("input row ".to_string()
                        + &(row_index + 1).to_string()
                        + " was not the same length as first row")
                        .into();
                    return Err(error);
                }
            };
            map.entry((Orientation::Vertical, char_index as i64))
                .and_modify(|string| string.push(c))
                .or_insert(c.to_string());
            map.entry((Orientation::Diagonal1, char_index  as i64 + *row_index as i64))
                .and_modify(|string| string.push(c))
                .or_insert(c.to_string());
            map.entry((Orientation::Diagonal2, char_index as i64 - *row_index as i64))
                .and_modify(|string| string.push(c))
                .or_insert(c.to_string());
        }
    }

    let vert_rows = map
        .iter()
        .filter(|(key, _val)| key.0 == Orientation::Vertical)
        .map(|(_key, val)| val.to_string())
        .collect::<Vec<_>>();

    let diag1_rows = map
        .iter()
        .filter(|(key, _val)| key.0 == Orientation::Diagonal1)
        .map(|(_key, val)| val.to_string())
        .collect::<Vec<_>>();

    let diag2_rows = map
        .iter()
        .filter(|(key, _val)| key.0 == Orientation::Diagonal2)
        .map(|(_key, val)| val.to_string())
        .collect::<Vec<_>>();

    Ok((input, vert_rows, diag1_rows, diag2_rows))
}

fn read_and_process_input(file_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut lines = vec![];

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

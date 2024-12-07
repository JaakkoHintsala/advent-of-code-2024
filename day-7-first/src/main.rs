use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use regex::Regex;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-7-first/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let sum: i64 = input
        .iter()
        .flat_map(|(result, values)| solve(result.clone(), values.clone()))
        .map(|solve_res| solve_res.0)
        .sum();

    println!("total sum: {}", sum);
}
#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
}

fn solve(result: i64, values: Vec<i64>) -> Option<(i64, Vec<i64>, Vec<Operation>)> {
    let mut rev = values.clone();
    rev.reverse();

    let res = solve_inner(result, rev, vec![]);
    Some((result, values, res?))
}

fn solve_inner(result: i64, values: Vec<i64>, answer: Vec<Operation>) -> Option<Vec<Operation>> {
    if result == 0 && values.iter().all(|value| *value == 0) {
        return Some(answer);
    }

    if result <= 0 {
        return None;
    }

    let (head, tail) = match values.split_first() {
        Some(val) => val.clone(),
        None => return None,
    };

    return solve_inner(
        result - head,
        tail.to_vec(),
        [vec![Operation::Add], answer.clone()].concat(),
    )
        .or_else(|| {
            if result % head != 0 {
                return None;
            }
            solve_inner(
                result / head,
                tail.to_vec(),
                [vec![Operation::Multiply], answer.clone()].concat(),
            )
        });
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<Vec<(i64, Vec<i64>)>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);
    let reg = Regex::new(r"(\d+):((?: \d+)+)").unwrap();

    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let mut parsed = vec![];

    for cap in reg.captures_iter(&input) {
        let result = &cap[1].to_string().parse::<i64>()?;
        let values = cap[2]
            .trim()
            .split_whitespace()
            .map(|int_str| int_str.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;
        parsed.push((result.clone(), values));
    }
    Ok(parsed)
}

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;
use std::vec;

use regex::Regex;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-13-first/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };
    // println!("{:?}", input);

    let res: u64 = input.iter().flat_map(|&a_b_price| get_solution(a_b_price)).sum();

    println!("fewest tokens: {}", res);
}

fn get_solution((a_x, a_y, b_x, b_y, price_x, price_y): (u64, u64, u64, u64, u64, u64)) -> Option<u64> {
    let mut solutions: Vec<(u64, u64)> = vec![];

    let a_x_range = (0..=price_x).step_by(a_x.try_into().unwrap());
    let a_y_range = (0..=price_y).step_by(a_y.try_into().unwrap());

    for (a_button_presses, (a_x_part, a_y_part)) in a_x_range.zip(a_y_range).enumerate() {
        let remaining_x = price_x - a_x_part;
        let remaining_y = price_y - a_y_part;

        if remaining_x % b_x != 0 || remaining_y % b_y != 0 {
            continue;
        }

        let b_button_presses_by_x = remaining_x / b_x;
        let b_button_presses_by_y = remaining_y / b_y;

        if b_button_presses_by_x == b_button_presses_by_y {
            solutions.push((a_button_presses.try_into().unwrap(), b_button_presses_by_x));
        }
    }

    solutions.iter().map(|&(a_presses, b_presses)|  a_presses * 3 + b_presses * 1).min()


}

fn read_and_process_input(
    file_path: &Path,
) -> Result<Vec<(u64, u64, u64, u64, u64, u64)>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut input_raw = String::new();
    reader.read_to_string(&mut input_raw)?;

    regex_parse(input_raw).map_err(|parse_err| parse_err.to_string().into())
}

fn regex_parse(input: String) -> Result<Vec<(u64, u64, u64, u64, u64, u64)>, ParseIntError> {
    let regex = Regex::new(r"Button A: X\+(?<A_X>\d+), Y\+(?<A_Y>\d+)\nButton B: X\+(?<B_X>\d+), Y\+(?<B_Y>\d+)\nPrize: X=(?<price_X>\d+), Y=(?<price_Y>\d+)")
    .unwrap();

    let mut parsed: Vec<(u64, u64, u64, u64, u64, u64)> = vec![];

    for (_, [a_x, a_y, b_x, b_y, price_x, price_y]) in
        regex.captures_iter(&input).map(|c| c.extract())
    {
        parsed.push((
            a_x.parse::<u64>()?,
            a_y.parse::<u64>()?,
            b_x.parse::<u64>()?,
            b_y.parse::<u64>()?,
            price_x.parse::<u64>()?,
            price_y.parse::<u64>()?,
        ));
    }

    Ok(parsed)
}

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

    let input_path = project_root_path.join(Path::new("day-13-second/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };
    //  println!("{:?}", input);

    let res: i64 = input
        .iter()
        .flat_map(|&a_b_price| solve(a_b_price))
        .sum();

    println!("fewest tokens: {}", res);
}

// a_presses * a_x + b_presses * b_x = p_x
// a_presses * a_x = p_x - b_presses * b_x
// a_presses = (p_x - b_presses * b_x) / a_x

// a_presses * a_y + b_presses * b_y = p_y

//p_y = ((p_x - b_presses * b_x) / a_x) * a_y + b_presses * b_y
//p_y = ((p_x - b_presses * b_x) * a_y / a_x) + b_presses * b_y
//p_y * a_x = (p_x - b_presses * b_x) * a_y + b_presses * b_y * a_x
//p_y * a_x = p_x * a_y - b_presses * b_x * a_y + b_presses * b_y * a_x
//p_y * a_x - p_x * a_y  = - b_presses * b_x * a_y + b_presses * b_y * a_x
//p_y * a_x - p_x * a_y  =  b_presses * (b_y * a_x - b_x * a_y)
//(p_y * a_x - p_x * a_y) / (b_y * a_x - b_x * a_y) = b_presses

// a_presses * a_y + b_presses * b_y = p_y
// a_presses * a_y = p_y - b_presses * b_y
// a_presses = (p_y - b_presses * b_y) / a_y

fn solve((a_x, a_y, b_x, b_y, price_x, price_y): (i64, i64, i64, i64, i64, i64)) -> Option<i64> {
    let b_presses_is_integer = (price_y * a_x - price_x * a_y) % (b_y * a_x - b_x * a_y) == 0;

    if !b_presses_is_integer {
        return None;
    }

    let b_presses = (price_y * a_x - price_x * a_y) / (b_y * a_x - b_x * a_y);

    let a_presses_is_integer = (price_y - b_presses * b_y) % a_y == 0;

    if !a_presses_is_integer {
        return None;
    }

    let a_presses = (price_y - b_presses * b_y) / a_y;

    Some(a_presses * 3 + b_presses)
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<Vec<(i64, i64, i64, i64, i64, i64)>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut input_raw = String::new();
    reader.read_to_string(&mut input_raw)?;

    regex_parse(input_raw).map_err(|parse_err| parse_err.to_string().into())
}

fn regex_parse(input: String) -> Result<Vec<(i64, i64, i64, i64, i64, i64)>, ParseIntError> {
    let regex = Regex::new(r"Button A: X\+(?<A_X>\d+), Y\+(?<A_Y>\d+)\nButton B: X\+(?<B_X>\d+), Y\+(?<B_Y>\d+)\nPrize: X=(?<price_X>\d+), Y=(?<price_Y>\d+)")
    .unwrap();

    let mut parsed: Vec<(i64, i64, i64, i64, i64, i64)> = vec![];

    for (_, [a_x, a_y, b_x, b_y, price_x, price_y]) in
        regex.captures_iter(&input).map(|c| c.extract())
    {
        parsed.push((
            a_x.parse::<i64>()?,
            a_y.parse::<i64>()?,
            b_x.parse::<i64>()?,
            b_y.parse::<i64>()?,
            price_x.parse::<i64>()? + 10000000000000,
            price_y.parse::<i64>()? + 10000000000000,
        ));
    }

    Ok(parsed)
}

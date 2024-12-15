use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-10-first/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    // println!("input {:?}", input);

    let res = get_trail_heads(&input)
        .iter()
        .flat_map(|head| {
            calculate_score(&input, (head.0, head.1, '0'))
                .into_iter()
                .unique()
        })
        .count();

    println!("total score: {}", res);
}

fn calculate_score(
    input: &HashMap<(usize, usize), char>,
    (x, y, c): (usize, usize, char),
) -> Vec<(usize, usize, char)> {
    if !c.is_ascii_digit() {
        return vec![];
    }

    if c.eq(&'9') {
        return vec![(x, y, c)];
    }

    let incremented = ((c as u8) + 1) as char;

    let up = if y == 0 {
        vec![]
    } else {
        let end_points = input
            .get(&(x, y - 1))
            .filter(|&&next| next.eq(&incremented))
            .into_iter()
            .flat_map(|&next| calculate_score(&input, (x, y - 1, next)))
            .collect();
        end_points
    };

    let right = input
        .get(&(x + 1, y))
        .filter(|&&next| next.eq(&incremented))
        .into_iter()
        .flat_map(|&next| calculate_score(&input, (x + 1, y, next)))
        .collect::<Vec<_>>();

    let down = input
        .get(&(x, y + 1))
        .filter(|&&next| next.eq(&incremented))
        .into_iter()
        .flat_map(|&next| calculate_score(&input, (x, y + 1, next)))
        .collect::<Vec<_>>();

    let left = if x == 0 {
        vec![]
    } else {
        let end_points = input
            .get(&(x - 1, y))
            .filter(|&&next| next.eq(&incremented))
            .into_iter()
            .flat_map(|&next| calculate_score(&input, (x - 1, y, next)))
            .collect();
        end_points
    };

    // println!("up {:?}", up);
    // println!("right {:?}", right);
    // println!("down {:?}", down);
    // println!("left {:?}", left);

    return vec![up, right, down, left].into_iter().flatten().collect();
}

fn get_trail_heads(input: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    input
        .clone()
        .into_iter()
        .filter(|&item| item.1.eq(&'0'))
        .map(|item| item.0)
        .collect::<Vec<_>>()
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<HashMap<(usize, usize), char>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut res: HashMap<(usize, usize), char> = HashMap::new();
    let lines = reader
        .lines()
        .into_iter()
        .map_ok(|s| s.chars().collect::<Vec<_>>())
        .collect::<Result<Vec<_>, _>>()?;

    let y_length = lines.len();

    let x_length = match lines.first() {
        Some(first) => first.len(),
        None => return Err("Empty input".into()),
    };

    for y in 0..y_length {
        for x in 0..x_length {
            let c = lines[y][x];
            res.insert((x, y), c);
        }
    }

    Ok(res)
}

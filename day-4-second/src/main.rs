use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root dir: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-4-second/input.txt"));

    let input: HashMap<(usize, usize), char> = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let x_mas_count = input
        .iter()
        .filter(|entry| entry.1.eq(&'A') && check_for_mas(&input, *entry.0))
        .count();

    println!("'X'-MAS count: {}", x_mas_count);
}

fn check_for_mas(
    input: &HashMap<(usize, usize), char>,
    (coord_row, coord_col): (usize, usize),
) -> bool {
    if coord_row == 0 || coord_col == 0 {
        return false;
    }

    let diag1 = input
        .get(&(coord_row - 1, coord_col - 1))
        .and_then(|first| {
            input
                .get(&(coord_row + 1, coord_col + 1))
                .map(|second| [first.to_string(), second.to_string()].concat())
        })
        .filter(|d| d.eq("MS") || d.eq("SM"));

    let diag2 = input
        .get(&(coord_row + 1, coord_col - 1))
        .and_then(|first| {
            input
                .get(&(coord_row - 1, coord_col + 1))
                .map(|second| [first.to_string(), second.to_string()].concat())
        })
        .filter(|d| d.eq("MS") || d.eq("SM"));

    return diag1.is_some() && diag2.is_some();
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<HashMap<(usize, usize), char>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut input = vec![];
    for line in reader.lines() {
        input.push(line?.chars().collect::<Vec<_>>());
    }

    let first_row_len = match input.first() {
        Some(str) => str.len(),
        None => return Err("input was empty".into()),
    };

    let mut char_coords: HashMap<(usize, usize), char> = HashMap::new();

    for row_ind in 0..first_row_len {
        for col_ind in 0..input.len() {
            let char_at_coord = match input.get(col_ind).and_then(|row| row.get(row_ind)) {
                Some(c) => c,
                None => {
                    return Err(
                        ("input length was not equal to first row at row: ".to_string()
                            + &col_ind.to_string())
                            .into(),
                    );
                }
            };
            char_coords.insert((row_ind, col_ind), char_at_coord.clone());
        }
    }

    Ok(char_coords)
}

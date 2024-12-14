use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-8-first/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let res = get_antennas_grouped(input.clone())
        .iter()
        .flat_map(|(_key, value)| get_possible_antinode_coords(value.clone()))
        .unique()
        .flat_map(|antinode_coord| input.get(&antinode_coord))
        .count();

    println!("antinode count: {}", res);
}

fn get_possible_antinode_coords(antenna_coords: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let res = antenna_coords
        .iter()
        .combinations(2)
        .flat_map(|pair| {
            let delta_x = pair[1].0 - pair[0].0;
            let delta_y = pair[1].1 - pair[0].1;
            let first = (pair[0].0 - delta_x, pair[0].1 - delta_y);
            let second = (pair[1].0 + delta_x, pair[1].1 + delta_y);
            vec![first, second]
        })
        .collect::<Vec<_>>();
    res
}

fn get_antennas_grouped(map: HashMap<(i64, i64), char>) -> HashMap<char, Vec<(i64, i64)>> {
    let mut ret: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    map.iter()
        .filter(|(_coords, value)| value.ne(&&'.'))
        .into_group_map_by(|(_coords, value)| **value)
        .iter()
        .for_each(|(key, value)| {
            ret.insert(*key, value.iter().map(|(coords, _c)| **coords).collect());
        });
    ret
}

fn make_coord_map(
    input: Vec<Vec<char>>,
) -> Result<HashMap<(i64, i64), char>, Box<dyn std::error::Error>> {
    let mut ret = HashMap::new();
    let row_count = input.len();
    for y in 0..row_count {
        let col_count = input[y].len();
        for x in 0..col_count {
            ret.insert((i64::try_from(x)?, i64::try_from(y)?), input[y][x]);
        }
    }
    Ok(ret)
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<HashMap<(i64, i64), char>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut parsed = vec![];
    for line_res in reader.lines() {
        let line = line_res?.trim().chars().collect();
        parsed.push(line);
    }
    make_coord_map(parsed)
}

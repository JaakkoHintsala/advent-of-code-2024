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

    let input_path = project_root_path.join(Path::new("day-8-second/input.txt"));

    let (input, x_max, y_max) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let res = get_antennas_grouped(input.clone())
        .iter()
        .flat_map(|(_key, value)| get_possible_antinode_coords_part2(value.clone(), ((x_max * y_max) / 2) + 1))
        .unique()
        .flat_map(|antinode_coord| input.get(&antinode_coord))
        .count();

    println!("antinode count: {}", res);
}

fn get_possible_antinode_coords_part2(
    antenna_coords: Vec<(i64, i64)>,
    max_iters: i64,
) -> Vec<(i64, i64)> {
    let res = antenna_coords
        .iter()
        .combinations(2)
        .flat_map(|pair| {
            let delta_x = pair[1].0 - pair[0].0;
            let delta_y = pair[1].1 - pair[0].1;
            let res = (0..max_iters)
                .flat_map(|iteration| {
                    vec![
                        (
                            pair[0].0 - (delta_x * iteration),
                            pair[0].1 - (delta_y * iteration),
                        ),
                        (
                            pair[1].0 + (delta_x * iteration),
                            pair[1].1 + (delta_y * iteration),
                        )
                    ]
                })
                .collect::<Vec<_>>();
            res
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
) -> Result<(HashMap<(i64, i64), char>, i64, i64), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut parsed = vec![];
    let mut x_max: usize = 0;
    for line_res in reader.lines() {
        let line = line_res?.trim().chars().collect::<Vec<_>>();
        x_max = line.len();
        parsed.push(line);
    }
    Ok((
        make_coord_map(parsed.clone())?,
        i64::try_from(x_max)?,
        i64::try_from(parsed.len())?,
    ))
}

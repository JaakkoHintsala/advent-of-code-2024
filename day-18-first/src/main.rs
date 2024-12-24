use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("test-inputs/18.txt"));

    let (input, size) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };
    dbg!(size);
    let goal: (i64, i64) = (size - 1, size - 1);

    let map = make_map(input.into_iter().take(1024).collect_vec(), size);
    let mut cur_paths: Vec<Vec<(i64, i64)>> = vec![vec![(0,0)]];
    let mut solution: Vec<(i64, i64)> = vec![];
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert((0, 0));
    loop {
        match advance_paths(&cur_paths, &mut visited, &map, goal) {
            AdvanceResult::CurPaths(paths) => { 
                cur_paths = paths;
             },
            AdvanceResult::Found(found) => {
                solution = found;
                break;
            },AdvanceResult::DeadEnd => {
                break;
            },
        }
    }


    let res = solution.len() - 1;

    println!("Solution length: {:?}", res);
}

enum AdvanceResult {
    Found(Vec<(i64, i64)>),
    CurPaths(Vec<Vec<(i64, i64)>>),
    DeadEnd
}

fn advance_paths(
    paths: &Vec<Vec<(i64, i64)>>,
    visited: &mut HashSet<(i64, i64)>,
    map: &HashMap<(i64, i64), char>,
    goal: (i64, i64),
) -> AdvanceResult {
    let mut advanced = vec![];
    if paths.is_empty() {
        return AdvanceResult::DeadEnd;
    }
    for path in paths {
        let cur_location = path.last().unwrap().clone();

        if cur_location.eq(&goal) {
            return AdvanceResult::Found(path.clone());
        }

        for next_coord in get_next_coords(cur_location.0, cur_location.1) {
            // dbg!(next_coord);
            if map.get(&next_coord).filter(|&&ch| ch.eq(&'.')).is_none() {
                // println!("bruh1");
                continue;
            }
            if visited.contains(&next_coord) {
                // println!("bruh2");
                continue;
            }
            let additional_element = std::iter::once(next_coord);
            let updated_path = path.iter().copied().chain(additional_element).collect_vec();
            visited.insert(next_coord);
            advanced.push(updated_path);
        }
    }
    AdvanceResult::CurPaths(advanced)
}
fn get_next_coords(x: i64, y: i64) -> Vec<(i64, i64)> {
    return vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
}

fn make_map(input: Vec<(i64, i64)>, size: i64) -> HashMap<(i64, i64), char> {
    let mut res = HashMap::new();
    for y in 0..size {
        for x in 0..size {
            res.insert((x, y), '.');
        }
    }
    for obstacle in input {
        res.insert(obstacle, '#');
    }
    res
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<(Vec<(i64, i64)>, i64), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut res = vec![];
    for line in lines {
        let splitted = line.trim().split(',').collect::<Vec<_>>();
        let x = splitted[0].parse::<i64>().unwrap();
        let y = splitted[1].parse::<i64>().unwrap();
        res.push((x, y));
    }
    let is_big = res.iter().find(|(x, y)| *x >= 7 || *y >= 7).is_some();
    let size: i64 = if is_big { 71 } else { 7 };
    return Ok((res, size));
}

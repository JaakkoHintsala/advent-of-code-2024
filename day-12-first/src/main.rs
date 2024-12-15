use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-12-first/input.txt"));

    let input: HashMap<(usize, usize), char> = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let mut processed: HashMap<(usize, usize), (char, usize, HashSet<(usize, usize)>)> =
        HashMap::new();

    for (&coords, &c) in &input {
        if processed.contains_key(&coords) {
            continue;
        }

        let mut visited = HashSet::new();
        let perimeter = count_perimeter(&mut visited, (coords, c), &input);
        for &visited_coord in &visited {
            processed.insert(visited_coord, (c, perimeter, visited.clone()));
        }
    }

    // println!("processed {:?}", processed);

    let res: usize = processed
        .iter()
        .unique_by(|item| item.1.2.iter().copied().collect::<Vec<_>>())
        .map(|item| item.1.1 * item.1.2.len())
        .sum();

    println!("total price: {}", res);
}

fn count_perimeter(
    visited: &mut HashSet<(usize, usize)>,
    (cur_pos, c): ((usize, usize), char),
    coord_map: &HashMap<(usize, usize), char>,
) -> usize {
    if visited.contains(&cur_pos) {
        return 0;
    }

    visited.insert(cur_pos);

    let x_minus = cur_pos.0.checked_sub(1);
    let y_minus = cur_pos.1.checked_sub(1);

    let up = y_minus
        .and_then(|y| {
            let next_coords = (cur_pos.0, y);
            coord_map
                .get(&next_coords)
                .filter(|&&next_char| next_char.eq(&c))
                .copied()
                .map(|next_char| count_perimeter(visited, (next_coords, next_char), coord_map))
        })
        .unwrap_or(1);

    let next_coords_plus_x = (cur_pos.0 + 1, cur_pos.1);

    let right = coord_map
        .get(&next_coords_plus_x)
        .filter(|&&next_char| next_char.eq(&c))
        .copied()
        .map(|next_char| count_perimeter(visited, (next_coords_plus_x, next_char), coord_map))
        .unwrap_or(1);

    let next_coords_plus_y = (cur_pos.0, cur_pos.1 + 1);

    let down = coord_map
        .get(&next_coords_plus_y)
        .filter(|&&next_char| next_char.eq(&c))
        .copied()
        .map(|next_char| count_perimeter(visited, (next_coords_plus_y, next_char), coord_map))
        .unwrap_or(1);

    let left = x_minus
        .and_then(|x| {
            let next_coords = (x, cur_pos.1);
            coord_map
                .get(&next_coords)
                .filter(|&&next_char| next_char.eq(&c))
                .copied()
                .map(|next_char| count_perimeter(visited, (next_coords, next_char), coord_map))
        })
        .unwrap_or(1);

    return up + right + down + left;
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

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

    let input_path = project_root_path.join(Path::new("day-12-second/input.txt"));

    let (input, x_length, y_length) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let mut regions: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for (&coords, &c) in &input {
        if regions.contains_key(&coords) {
            continue;
        }

        let mut visited = HashSet::new();
        count_perimeter(&mut visited, (coords, c), &input);
        for &visited_coord in &visited {
            regions.insert(visited_coord, visited.clone());
        }
    }
    let mut sides: HashMap<(usize, usize), u64> = HashMap::new();
    let mut previous_top_side = false;
    let mut previous_bottom_side = false;
    for y_ind in 0..y_length {
        for x_ind in 0..x_length {
            let c = input.get(&(x_ind, y_ind)).unwrap();

            let is_up_a_side = y_ind
                .checked_sub(1)
                .and_then(|y_minus| {
                    let next_coords = (x_ind, y_minus);
                    input
                        .get(&next_coords)
                        .filter(|&&next_char| next_char.eq(&c))
                })
                .is_none();

            let next_coords = (x_ind, y_ind + 1);

            let is_down_a_side = input
                .get(&next_coords)
                .filter(|&&next_char| next_char.eq(&c))
                .is_none();

            let sides_to_add = (!previous_top_side && is_up_a_side) as u64
                + (!previous_bottom_side && is_down_a_side) as u64;

            sides
                .entry((x_ind, y_ind))
                .and_modify(|old| *old = *old + sides_to_add)
                .or_insert(sides_to_add);

            // println!("y_ind {:?}", y_ind);
            // println!("x_ind {:?}", x_ind);
            // println!("c {:?}", c);
            // println!("is_up_a_side {:?}", is_up_a_side);
            // println!("previous_top_side {:?}", previous_top_side);
            // println!("is_down_a_side {:?}", is_down_a_side);
            // println!("previous_bottom_side {:?}", previous_bottom_side);
            // println!("sides_to_add {:?}\n\n\n", sides_to_add);

            let next_in_line = input
                .get(&(x_ind + 1, y_ind))
                .filter(|&&next_char| next_char.eq(&c));
            previous_top_side = is_up_a_side && next_in_line.is_some();
            previous_bottom_side = is_down_a_side && next_in_line.is_some();
        }
        previous_top_side = false;
        previous_bottom_side = false;
    }

    let mut previous_left_side = false;
    let mut previous_right_side = false;
    for x_ind in 0..x_length {
        for y_ind in 0..y_length {
            let c = input.get(&(x_ind, y_ind)).unwrap();

            let is_left_a_side = x_ind
                .checked_sub(1)
                .and_then(|x_minus| {
                    let next_coords = (x_minus, y_ind);
                    input
                        .get(&next_coords)
                        .filter(|&&next_char| next_char.eq(&c))
                })
                .is_none();

            let next_coords = (x_ind + 1, y_ind);

            let is_right_a_side = input
                .get(&next_coords)
                .filter(|&&next_char| next_char.eq(&c))
                .is_none();

            let sides_to_add = (!previous_left_side && is_left_a_side) as u64
                + (!previous_right_side && is_right_a_side) as u64;

            sides
                .entry((x_ind, y_ind))
                .and_modify(|old| *old = *old + sides_to_add)
                .or_insert(sides_to_add);

            let next_in_line = input
                .get(&(x_ind, y_ind + 1))
                .filter(|&&next_char| next_char.eq(&c));

            previous_left_side = is_left_a_side && next_in_line.is_some();
            previous_right_side = is_right_a_side && next_in_line.is_some();
        }
        previous_left_side = false;
        previous_right_side = false;
    }

    let res: u64 = regions
        .iter()
        .unique_by(|item| item.1.iter().copied().collect::<Vec<_>>())
        .map(|item| {
            let side_count: u64 = item.1.iter().flat_map(|coord| sides.get(coord)).sum();
            side_count * (item.1.len() as u64)
        })
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
) -> Result<(HashMap<(usize, usize), char>, usize, usize), Box<dyn std::error::Error>> {
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

    Ok((res, lines.first().unwrap().len(), lines.len()))
}

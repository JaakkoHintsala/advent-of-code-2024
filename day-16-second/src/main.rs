use std::io::{self, BufRead};
use std::usize;
use std::{collections::HashMap, fs::File, path::Path};

use itertools::Itertools;

static mut DIMS: (usize, usize) = (0, 0);

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("test-inputs/16.txt"));

    let (maze, ((start_x, start_y), start_ch), ((end_x, end_y), _), dimensions) =
        match read_and_process_input(input_path.as_path()) {
            Ok(valid_input) => valid_input,
            Err(error) => {
                eprintln!("Error while reading input: {}", error);
                std::process::exit(1);
            }
        };
    unsafe { DIMS = dimensions };

    let paths = find_paths(
        &maze,
        &mut HashMap::new(),
        ((start_x, start_y), start_ch, Direction::Right, 0),
        &HashMap::new(),
        (end_x, end_y),
    );
    let binding = paths.iter().into_group_map_by(|path| score_path(&path));

    let paths_rated = binding
        .iter()
        .min_by_key(|entry| *entry.0)
        .iter()
        .map(|entry| entry.1)
        .flatten()
        .collect_vec();

    let seats: HashMap<_, _> = paths_rated.into_iter().flat_map(|map| map.iter()).collect();

    println!("seats {}", seats.iter().len());

    // for path in &paths_rated {
    //     pretty_print(&maze, &path.0);
    //     println!("Rating: {}", path.1);
    //     println!("\n\n");
    // }
}

fn score_path(path: &HashMap<(usize, usize), (char, Direction, u8)>) -> usize {
    let steps = path.len() - 1;
    let total_turns: usize = path
        .iter()
        .map(|(_, &(_, _, turns))| 1000usize * turns as usize)
        .sum();
    steps + total_turns
}

fn find_paths(
    maze: &HashMap<(usize, usize), char>,
    best_weigths: &mut HashMap<(usize, usize, Direction), usize>,
    ((cur_location_x, cur_location_y), cur_char, cur_direction, previous_turns_taken): (
        (usize, usize),
        char,
        Direction,
        u8,
    ),
    path_so_far: &HashMap<(usize, usize), (char, Direction, u8)>,
    (end_x, end_y): (usize, usize),
) -> Vec<HashMap<(usize, usize), (char, Direction, u8)>> {
    let mut updated_path = path_so_far.clone();
    updated_path.insert(
        ((cur_location_x), (cur_location_y)),
        ((cur_char), (cur_direction), (previous_turns_taken)),
    );

    // pretty_print(maze, &updated_path);

    let cur_score = score_path(&updated_path);
    let best_over_all = best_weigths
        .get(&(cur_location_x, cur_location_y, cur_direction))
        .unwrap_or(&usize::MAX);

    // println!("best_over_all {}", &best_over_all);
    // println!("cur_score {}", cur_score);

    if *(best_over_all) < cur_score {
        // println!("bruh1");
        return vec![];
    }

    best_weigths.insert((cur_location_x, cur_location_y, cur_direction), cur_score);

    let goal_score = vec![
        best_weigths.get(&(end_x, end_y, Direction::Up)),
        best_weigths.get(&(end_x, end_y, Direction::Right)),
        best_weigths.get(&(end_x, end_y, Direction::Down)),
        best_weigths.get(&(end_x, end_y, Direction::Left)),
    ]
    .into_iter()
    .flatten()
    .min_by_key(|tripl| **tripl)
    .unwrap_or(&usize::MAX);

    if *(goal_score) < cur_score {
        // println!("bruh2");
        return vec![];
    }

    // println!("cur_location_x {} cur_location_y {} cur_char {} ", cur_location_x, cur_location_y, cur_char );
    // println!("path_so_far {:?} ", path_so_far );
    // println!("updated_path {:?} ", updated_path );
    // println!("goal score {}", goal_score);

    // pretty_print(maze, &updated_path);
    if cur_char.eq(&'E') {
        // println!("goal score {}", goal_score);
        return vec![updated_path];
    }

    let up_coords: Option<((usize, usize), Direction)> = Some(cur_location_x)
        .zip(cur_location_y.checked_add_signed(-1))
        .zip(Some(Direction::Up));
    let right_coords = cur_location_x
        .checked_add_signed(1)
        .zip(Some(cur_location_y))
        .zip(Some(Direction::Right));
    let down_coords = Some(cur_location_x)
        .zip(cur_location_y.checked_add_signed(1))
        .zip(Some(Direction::Down));
    let left_coords = cur_location_x
        .checked_add_signed(-1)
        .zip(Some(cur_location_y))
        .zip(Some(Direction::Left));

    return vec![up_coords, right_coords, down_coords, left_coords]
        .iter()
        .flatten()
        .flat_map(|&(coords, dir)| maze.get_key_value(&coords).zip(Some(dir)))
        .filter(|&((_, &ch), _)| ch.eq(&'.') || ch.eq(&'E') || ch.eq(&'O'))
        .filter(|&((coords, _), _)| path_so_far.get(coords).is_none())
        .flat_map(|((next_coords, ch), dir)| {
            return find_paths(
                maze,
                best_weigths,
                (*next_coords, *ch, dir, cur_direction.turns_to(dir)),
                &updated_path,
                (end_x, end_y),
            );
        })
        .collect_vec();
}

fn _is_valid_target_coords(
    maze: &HashMap<(usize, usize), char>,
    coords: (usize, usize),
    path_so_far: &HashMap<(usize, usize), (char, Direction, u8)>,
) -> bool {
    return maze
        .get(&coords)
        .filter(|&ch| ch.eq(&'.') || ch.eq(&'E') || ch.eq(&'O'))
        .is_some()
        && path_so_far.get(&coords).is_none();
}

fn _pretty_print(
    area: &HashMap<(usize, usize), char>,
    path: &HashMap<(usize, usize), (char, Direction, u8)>,
) {
    let (x_length, y_length) = unsafe { DIMS };
    let mut area_to_print = area.clone();

    for step in path {
        area_to_print.insert(*step.0, step.1.1._into_char());
    }

    let mut formatted = String::new();
    for y in 0..y_length {
        for x in 0..x_length {
            area_to_print.get(&(x, y)).inspect(|c| formatted.push(**c));
        }
        formatted.push('\n');
    }
    print!("{}", formatted);
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turns_to(self, other: Direction) -> u8 {
        let right_turns = {
            let current = self as isize;
            let target = other as isize;
            ((target + 4) - current) as u8 % 4
        };

        let left_turns = {
            let current = 3 - self as isize;
            let target = 3 - other as isize;
            ((target + 4) - current) as u8 % 4
        };

        let ret = right_turns.min(left_turns);

        ret
    }
}

impl Direction {
    fn _into_char(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            // Direction::Up => 'O',
            // Direction::Right => 'O',
            // Direction::Down => 'O',
            // Direction::Left => 'O',
        }
    }
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<
    (
        HashMap<(usize, usize), char>,
        ((usize, usize), char),
        ((usize, usize), char),
        (usize, usize),
    ),
    Box<dyn std::error::Error>,
> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut maze: HashMap<(usize, usize), char> = HashMap::new();
    let lines = reader
        .lines()
        .into_iter()
        .map_ok(|s| s.trim().chars().collect::<Vec<_>>())
        .collect::<Result<Vec<_>, _>>()?;

    let maze_lines = lines
        .iter()
        .filter(|line| line.first().filter(|&&c| c.eq(&'#')).is_some())
        .collect::<Vec<_>>();

    let y_length = maze_lines.len();

    let x_length = match maze_lines.first() {
        Some(first) => first.len(),
        None => return Err("Empty input".into()),
    };

    for y in 0..y_length {
        for x in 0..x_length {
            let c = maze_lines[y][x];
            maze.insert((x, y), c);
        }
    }

    let start_location: ((usize, usize), char) = maze
        .clone()
        .into_iter()
        .find(|entry| entry.1.eq(&'S'))
        .ok_or(Into::<Box<dyn std::error::Error>>::into(
            "Start location not found",
        ))?;

    let end_location: ((usize, usize), char) = maze
        .clone()
        .into_iter()
        .find(|entry| entry.1.eq(&'E'))
        .ok_or(Into::<Box<dyn std::error::Error>>::into(
            "End location not found",
        ))?;

    Ok((maze, start_location, end_location, (x_length, y_length)))
}

use std::io::{self, BufRead};
use std::{collections::HashMap, fs::File, path::Path};

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("test-inputs/15.txt"));

    let (robot_area, commands, robot_starting_location, area_dimensions) =
        match read_and_process_input(input_path.as_path()) {
            Ok(valid_input) => valid_input,
            Err(error) => {
                eprintln!("Error while reading input: {}", error);
                std::process::exit(1);
            }
        };

    let mut cur_robot_area = robot_area.clone();
    let mut cur_pos = robot_starting_location.0;
    for command in commands {
        pretty_print(&cur_robot_area, area_dimensions);
        if let Some(next_pos) = move_robot_or_box(&mut cur_robot_area, (cur_pos, '@'), command) {
            cur_pos = next_pos;
        };
    }
    pretty_print(&cur_robot_area, area_dimensions);


    println!("sum_gps_coordinates: {}", sum_gps_coordinates(cur_robot_area));
}

fn pretty_print(robot_area: &HashMap<(usize, usize), char>, (x_length, y_length): (usize, usize)) {
    let mut formatted = String::new();
    for y in 0..y_length {
        for x in 0..x_length {
            robot_area.get(&(x, y)).inspect(|c| formatted.push(**c));
        }
        formatted.push('\n');
    }
    print!("{}", formatted);
}

fn sum_gps_coordinates(robot_area: HashMap<(usize, usize), char>) -> usize {
    robot_area
        .iter()
        .filter(|entry| entry.1.eq(&'O'))
        .map(|entry| entry.0.0 + entry.0.1 * 100)
        .sum()
}

fn move_robot_or_box(
    robot_area: &mut HashMap<(usize, usize), char>,
    ((source_x, source_y), source_char): ((usize, usize), char),
    direction: char,
) -> Option<(usize, usize)> {
    let target_coords = match direction {
        '^' => source_y
            .checked_add_signed(-1)
            .map(|incremented| (source_x, incremented)),
        '>' => source_x
            .checked_add_signed(1)
            .map(|incremented| (incremented, source_y)),
        'v' => source_y
            .checked_add_signed(1)
            .map(|incremented| (source_x, incremented)),
        '<' => source_x
            .checked_add_signed(-1)
            .map(|incremented| (incremented, source_y)),
        _ => None,
    }?;

    let char_at_target = robot_area.get(&target_coords)?;

    let valid_target_coords_opt = match char_at_target {
        '.' => Some(target_coords),
        'O' => {
            let successful_move =
                move_robot_or_box(robot_area, (target_coords, *char_at_target), direction);

                // println!("target_coords: {:?}", target_coords);
                // println!("successful_move: {:?}", successful_move);
            if successful_move.is_some() {
                 Some(target_coords)
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(valid_target_coords) = valid_target_coords_opt {
        robot_area.insert((source_x, source_y), '.');
        robot_area.insert(valid_target_coords, source_char);
        return valid_target_coords_opt;
    }
    return None;

}

fn read_and_process_input(
    file_path: &Path,
) -> Result<
    (
        HashMap<(usize, usize), char>,
        Vec<char>,
        ((usize, usize), char),
        (usize, usize),
    ),
    Box<dyn std::error::Error>,
> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut robot_area: HashMap<(usize, usize), char> = HashMap::new();
    let lines = reader
        .lines()
        .into_iter()
        .map_ok(|s| s.trim().chars().collect::<Vec<_>>())
        .collect::<Result<Vec<_>, _>>()?;

    let robot_area_lines = lines
        .iter()
        .filter(|line| line.first().filter(|&&c| c.eq(&'#')).is_some())
        .collect::<Vec<_>>();

    let y_length = robot_area_lines.len();

    let x_length = match robot_area_lines.first() {
        Some(first) => first.len(),
        None => return Err("Empty input".into()),
    };

    for y in 0..y_length {
        for x in 0..x_length {
            let c = robot_area_lines[y][x];
            robot_area.insert((x, y), c);
        }
    }

    let commands = lines
        .iter()
        .filter(|line| {
            line.first()
                .filter(|&&c| c.eq(&'^') || c.eq(&'>') || c.eq(&'v') || c.eq(&'<'))
                .is_some()
        })
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    let start_robot_location: ((usize, usize), char) = robot_area
        .clone()
        .into_iter()
        .find(|entry| entry.1.eq(&'@'))
        .ok_or(Into::<Box<dyn std::error::Error>>::into(
            "Robot start location not found",
        ))?;

    Ok((
        robot_area,
        commands,
        start_robot_location,
        (x_length, y_length),
    ))
}

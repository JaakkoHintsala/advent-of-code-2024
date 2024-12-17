use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;
use std::vec;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("test-inputs/14.txt"));

    let (dimensions, robot_data) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };
    // println!("\n\ndimensions {:?}", dimensions);
    // println!("\n\nrobot_data {:?}", robot_data);
    let robot_area = RobotArea {
        robot_data,
        dimensions,
    };

    let after_n_steps = robot_area.do_n_steps(100);
    // println!("\n\nafter_n_steps {}", format!("{:?}", after_n_steps).replace(", ", "\n"));

    println!("safety factor: {}", after_n_steps.mul_quadrants());
}
#[derive(Debug, Clone)]
struct RobotArea {
    robot_data: Vec<RobotData>,
    dimensions: (u32, u32),
}

#[derive(Debug, Clone, Copy)]
struct RobotData {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl From<(u32, u32, i64, i64)> for RobotData {
    fn from(data: (u32, u32, i64, i64)) -> Self {
        Self {
            pos_x: data.0.into(),
            pos_y: data.1.into(),
            vel_x: data.2,
            vel_y: data.3,
        }
    }
}

impl RobotArea {
    fn do_n_steps(&self, n: u32) -> RobotArea {
        let next_data = self
            .robot_data
            .iter()
            .map(|&data| {
                // return ((pos_x + vel_x * n) % self.dimensions.0);
                let next_x = (data.vel_x * Into::<i64>::into(n) + data.pos_x)
                    .rem_euclid(Into::<i64>::into(self.dimensions.0));
                let next_y = (data.vel_y * Into::<i64>::into(n) + data.pos_y)
                    .rem_euclid(Into::<i64>::into(self.dimensions.1));
                return RobotData {
                    pos_x: next_x,
                    pos_y: next_y,
                    ..data
                };
            })
            .sorted_by_key(|data| data.pos_y * Into::<i64>::into(self.dimensions.0) + data.pos_x)
            .collect::<Vec<_>>();

        RobotArea {
            robot_data: next_data,
            ..self.clone()
        }
    }
}

impl RobotArea {
    fn _pretty_print(&self) {
        let grouped = self
            .robot_data
            .iter()
            .into_group_map_by(|&&data| (data.pos_x, data.pos_y));
        let mut formatted = String::new();
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let char_to_print = grouped
                    .get(&(x.into(), y.into()))
                    .map(|robots| (robots.len() % 10).to_string().chars().nth(0).unwrap())
                    .unwrap_or('.');
                formatted.push(char_to_print);
            }
            formatted.push('\n');
        }
        print!("{}", formatted);
    }
}

impl RobotArea {
    fn count_robots(&self, from_x: u32, until_x: u32, from_y: u32, until_y: u32) -> usize {
        // println!("\n\n\nfrom_x {}", from_x);
        // println!("until_x {}", until_x);
        // println!("from_y {}", from_y);
        // println!("until_y {}", until_y);
        let sum = self
            .robot_data
            .iter()
            .filter(|&&data| {
                let ret = data.pos_x >= from_x.into()
                    && data.pos_x < until_x.into()
                    && data.pos_y >= from_y.into()
                    && data.pos_y < until_y.into();
                // println!("data: {:?}, hit: {}", data, ret);
                ret
            })
            .count();
        return sum;
    }

    fn mul_quadrants(&self) -> usize {
        return self.count_robots(0, self.dimensions.0 / 2, 0, self.dimensions.1 / 2)
            * self.count_robots(
                (self.dimensions.0 / 2) + 1,
                self.dimensions.0,
                0,
                self.dimensions.1 / 2,
            )
            * self.count_robots(
                0,
                self.dimensions.0 / 2,
                (self.dimensions.1 / 2) + 1,
                self.dimensions.1,
            )
            * self.count_robots(
                (self.dimensions.0 / 2) + 1,
                self.dimensions.0,
                (self.dimensions.1 / 2) + 1,
                self.dimensions.1,
            );
    }
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<((u32, u32), Vec<RobotData>), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut input_raw = String::new();
    reader.read_to_string(&mut input_raw)?;

    let parsed: Result<((u32, u32), Vec<RobotData>), Box<dyn std::error::Error>> =
        regex_parse(&input_raw)
            .and_then(|robot_data| {
                regex_parse_bathroom_dimensions(&input_raw)
                    .map(|dimensions| (dimensions, robot_data))
            })
            .map_err(|parse_err| parse_err.to_string().into())
            .and_then(|(dims_opt, robot_data)| match dims_opt {
                Some(dims) => match dims {
                    (x, y) if x <= 1 || y <= 1 => {
                        Err("robot area dimensions must be larger than 1".into())
                    }
                    (x, y) if x % 2 != 1 || y % 2 != 1 => {
                        Err("robot area dimensions must be uneven".into())
                    }
                    dims => Ok((dims, robot_data)),
                },
                None => Err("robot area dimensions not specified".into()),
            });

    parsed
}

fn regex_parse(input: &String) -> Result<Vec<RobotData>, ParseIntError> {
    let regex = Regex::new(r"p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)").unwrap();

    let mut parsed: Vec<RobotData> = vec![];

    for (_, [p_x, p_y, v_x, v_y]) in regex.captures_iter(&input).map(|c| c.extract()) {
        parsed.push(
            (
                p_x.parse::<u32>()?,
                p_y.parse::<u32>()?,
                v_x.parse::<i64>()?,
                v_y.parse::<i64>()?,
            )
                .into(),
        );
    }

    Ok(parsed)
}

fn regex_parse_bathroom_dimensions(input: &String) -> Result<Option<(u32, u32)>, ParseIntError> {
    let regex = Regex::new(r"robot_area_size=(?<x>\d+),(?<y>\d+)").unwrap();

    if let Some((_, [x, y])) = regex.captures(&input).map(|c| c.extract()) {
        return Ok(Some((x.parse::<u32>()?, y.parse::<u32>()?)));
    }
    Ok(None)
}

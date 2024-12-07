use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::num::ParseIntError;
use std::path::Path;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use strum_macros::EnumIter;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-6-second/input.txt"));

    let (input, width, height) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let start_state = EnvMap::new(height, width, input);

    let perms = start_state.get_block_perms();
    let loop_count =  perms.par_iter().map(|env| {
        let res = EnvMap::run(env);
        if res.eq(&EndState::Loop) {
            env.print();
        }
        res
    }).filter(|end_state| end_state.eq(&EndState::Loop)).count();
    println!("total loop permutations: {}", loop_count);


}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct EnvMap {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    history: HashSet<(usize, usize, char)>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum EndState {
    Loop,
    GuardOffTheMap,
}

impl EnvMap {
    fn new(height: usize, width: usize, data: Vec<Vec<char>>) -> Self {
        let history = HashSet::new();
        EnvMap {
            data,
            width,
            height,
            history,
        }
    }

    fn set(&mut self, row: usize, col: usize, value: char) {
        if let Some(row_vec) = self.data.get_mut(row) {
            if let Some(cell) = row_vec.get_mut(col) {
                *cell = value;
            }
        }
    }

    fn find_guard(&self) -> Result<(usize, usize, Direction), EndState> {
        for x in 0..self.width {
            for y in 0..self.height {
                let cur = self.get(x, y).map(|c| c);
                match cur {
                    Some('^') => return Ok((x, y, Direction::Up)),
                    Some('>') => return Ok((x, y, Direction::Right)),
                    Some('v') => return Ok((x, y, Direction::Down)),
                    Some('<') => return Ok((x, y, Direction::Left)),
                    _ => {}
                }
            }
        }
        Err(EndState::GuardOffTheMap)
    }

    fn is_blocked(&self, row: usize, col: usize, dir: Direction) -> bool {
        let res = match dir {
            Direction::Up => {
                if row == 0 {
                    return false;
                }
                self.get(row - 1, col)
                    .filter(|c| c.eq(&'#') || c.eq(&'O'))
                    .is_some()
            }
            Direction::Right => self
                .get(row, col + 1)
                .filter(|c| c.eq(&'#') || c.eq(&'O'))
                .is_some(),
            Direction::Down => self
                .get(row + 1, col)
                .filter(|c| c.eq(&'#') || c.eq(&'O'))
                .is_some(),
            Direction::Left => {
                if col == 0 {
                    return false;
                }
                self.get(row, col - 1)
                    .filter(|c| c.eq(&'#') || c.eq(&'O'))
                    .is_some()
            }
        };
        res
    }

    fn rotate(&self, dir: Direction) -> char {
        match dir {
            Direction::Up => '>',
            Direction::Right => 'v',
            Direction::Down => '<',
            Direction::Left => '^',
        }
    }

    fn add_to_history(&mut self, row: usize, col: usize, dir: char) -> bool {
        self.history.insert((row, col, dir))
    }

    fn move_guard(&self) -> Result<EnvMap, EndState> {
        let (row, col, dir) = self.find_guard()?;
        let mut clone = self.clone();
        clone.set(row, col, 'X');

        if self.is_blocked(row, col, dir.clone()) {
            let new_dir = self.rotate(dir.clone());

            if !clone.add_to_history(row, col, new_dir) {
                return Err(EndState::Loop);
            }

            clone.set(row, col, new_dir);

            return Ok(clone);
        }

        match dir {
            Direction::Up => {
                if row != 0 {
                    clone.set(row - 1, col, '^');
                    if !clone.add_to_history(row - 1, col, '^') {
                        return Err(EndState::Loop);
                    }
                }
            }
            Direction::Right => {
                clone.set(row, col + 1, '>');
                if !clone.add_to_history(row, col + 1, '>') {
                    return Err(EndState::Loop);
                }
            }
            Direction::Down => {
                clone.set(row + 1, col, 'v');
                if !clone.add_to_history(row + 1, col, 'v') {
                    return Err(EndState::Loop);
                }
            }
            Direction::Left => {
                if col != 0 {
                    clone.set(row, col - 1, '<');
                    if !clone.add_to_history(row, col - 1, '<') {
                        return Err(EndState::Loop);
                    }
                }
            }
        };
        Ok(clone)
    }
    fn run(input: &EnvMap) -> EndState {
        let mut cur_res = Ok(input.clone());

        while let Ok(ref cur) = cur_res {
            let next = cur.move_guard();

            // cur.print();
            // if let Err(ref error) = next {
            // //     println!("total Xs: {}", cur.countX());
            // //     println!("End State: {:?}", error);
            // }
            cur_res = next;
        };

        cur_res.unwrap_err()
    }

    fn get_block_perms(&self) -> Vec<EnvMap> {
        let (guard_row, guard_col, dir) = match self.find_guard() {
            Ok(tuple) => tuple,
            Err(_) => return vec![],
        };

        let mut ret = vec![];

        for row in 0..self.width {
            for col in 0..self.height {
                if (row != guard_row || col != guard_col)  && self.get(row, col).filter(|c| !c.eq(&'#')).is_some() {
                    let mut perm = self.clone();
                    perm.set(row, col, 'O');
                    ret.push(perm);
                }
            }
        }

        ret
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.data
            .get(row)
            .and_then(|row_vec| row_vec.get(col).copied())
    }

    fn print(&self) {
        for row in &self.data {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    fn count_x(&self) -> usize {
        self.data
            .iter()
            .flatten()
            .filter(|c| c.to_owned().eq(&'X'))
            .count()
    }
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<(Vec<Vec<char>>, usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut input = vec![];
    for line in reader.lines() {
        input.push(line?.chars().collect::<Vec<_>>());
    }
    let width = input.first().unwrap().len();
    let height = input.len();

    Ok((input, width, height))
}

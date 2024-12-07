use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-6-first/input.txt"));

    let (input, width, height) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    EnvMap::new(height, width, input).run();
}

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
}

impl EnvMap {
    fn new(height: usize, width: usize, data: Vec<Vec<char>>) -> Self {
        EnvMap {
            data,
            width,
            height,
        }
    }

    fn set(&mut self, row: usize, col: usize, value: char) {
        if let Some(row_vec) = self.data.get_mut(row) {
            if let Some(cell) = row_vec.get_mut(col) {
                *cell = value;
            }
        }
    }

    fn find_guard(&self) -> Option<(usize, usize, Direction)> {
        for x in 0..self.width {
            for y in 0..self.height {
                let cur = self.get(x, y).map(|c| c);
                match cur {
                    Some('^') => return Some((x, y, Direction::Up)),
                    Some('>') => return Some((x, y, Direction::Right)),
                    Some('v') => return Some((x, y, Direction::Down)),
                    Some('<') => return Some((x, y, Direction::Left)),
                    _ => {}
                }
            }
        }
        None
    }

    fn is_blocked(&self, row: usize, col: usize, dir: &Direction) -> bool {
        let res = match dir {
            Direction::Up => {
                if row == 0 {
                    return false;
                }
                self.get(row - 1, col).filter(|c| c.eq(&'#')).is_some()
            }
            Direction::Right => self.get(row, col + 1).filter(|c| c.eq(&'#')).is_some(),
            Direction::Down => self.get(row + 1, col).filter(|c| c.eq(&'#')).is_some(),
            Direction::Left => {
                if col == 0 {
                    return false;
                }

                self.get(row, col - 1).filter(|c| c.eq(&'#')).is_some()
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

    fn move_guard(&self) -> Option<EnvMap> {
        let (row, col, dir) = self.find_guard()?;
        let mut clone = self.clone();
        clone.set(row, col, 'X');

        if self.is_blocked(row, col, &dir) {
            clone.set(row, col, self.rotate(dir));

            return Some(clone);
        }

        match dir {
            Direction::Up => {
                if row != 0 {
                    clone.set(row - 1, col, '^');
                }
            }
            Direction::Right => {
                clone.set(row, col + 1, '>');
            }
            Direction::Down => {
                clone.set(row + 1, col, 'v');
            }
            Direction::Left => {
                if col != 0 {
                    clone.set(row, col - 1, '<');
                }
            }
        };
        Some(clone)
    }
    fn run(&self) {
        let mut cur_opt = Some(self.clone());

        while let Some(ref cur) = cur_opt {
            let next_opt = cur.move_guard();

            if next_opt.is_none() {
                cur.print();
                println!("total Xs: {}", cur.count_x());
            }

            cur_opt = next_opt;
        }
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

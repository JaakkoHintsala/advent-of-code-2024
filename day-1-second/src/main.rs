use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root dir: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-1-second/input.txt"));
    let input_path_str = input_path.to_str().unwrap_or_default();

    // print!("{}", input_path_str);

    let input = match read_and_process_input(input_path_str) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let (vec_a, vec_b) = input
        .iter()
        .map(|arr| (arr[0], arr[1]))
        .unzip::<_, _, Vec<u32>, Vec<u32>>();

    let mut map: HashMap<u32, u32> = HashMap::new();

    for vec_b_val in vec_b {
        map.entry(vec_b_val)
            .and_modify(|count| {
                *count = *count + 1u32;
            })
            .or_insert(1u32);
    }

    let mut summa: u32 = 0;

    for vec_a_val in vec_a {
        summa = summa + vec_a_val * map.get(&vec_a_val).unwrap_or_else(|| &0u32);
    }

    // let summa :u32 = map.iter().map( |entry| entry.1).sum();
    print!("{}", summa)
}

fn read_and_process_input(file_path: &str) -> Result<Vec<[u32; 2]>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut input: Vec<[u32; 2]> = vec![];

    let mut line_number = 1;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let my_array_res: Result<[u32; 2], _> = numbers.try_into().map_err(|_| {
            let error: Box<dyn std::error::Error> =
                ("Could not parse row ".to_owned() + &line_number.to_string()).into();
            error
        });

        let my_array = my_array_res?;
        input.push(my_array);
        line_number += 1;
    }

    Ok(input)
}

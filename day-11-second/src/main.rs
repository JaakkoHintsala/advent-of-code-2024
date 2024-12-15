use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-11-first/input.txt"));

    let input: Vec<u64> = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let mut count_map: HashMap<(u64, u64), u64> = HashMap::new();

    let res: u64 = input
        .iter()
        .map(|&item| recurse_dynamic(&mut count_map, item, 75))
        .sum();

    println!("Number of stones: {}", res);
}

fn _recurse(input: Vec<u64>, depth: u64) -> Vec<u64> {
    // println!("{:?}" , input);
    if depth == 0 {
        return input;
    }
    return _recurse(
        input.iter().flat_map(|&item| do_iteration(item)).collect(),
        depth - 1,
    );
}

fn recurse_dynamic(count_map: &mut HashMap<(u64, u64), u64>, input: u64, depth: u64) -> u64 {
    // println!("{:?}" , input);

    if depth == 0 {
        count_map.insert((input, depth), 1);
        return 1;
    }

    let memoized_count = count_map.get(&(input, depth));
    let real_count = match memoized_count {
        Some(&from_map) => from_map,
        None => {
            let count_recursed = do_iteration(input)
                .iter()
                .map(|&next_val| recurse_dynamic(count_map, next_val, depth - 1))
                .sum();
            count_map.insert((input, depth), count_recursed);
            count_recursed
        }
    };

    real_count
}

fn do_iteration(input: u64) -> Vec<u64> {
    match input {
        0 => vec![1],
        n if n.to_string().len() % 2 == 0 => split(n),
        _ => vec![input * 2024],
    }
}

fn split(input: u64) -> Vec<u64> {
    let as_str = input.to_string();
    let (first, sec) = as_str.split_at(as_str.len() / 2);
    let res = vec![first, sec]
        .iter()
        .map(|num_str| num_str.parse::<u64>())
        .collect::<Result<Vec<_>, _>>();

    if let Err(parse_err) = &res {
        println!("parse error: {:?}", parse_err);
    }

    res.iter().flatten().copied().collect::<Vec<_>>()
}

fn read_and_process_input(file_path: &Path) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut input_raw = String::new();
    reader.read_to_string(&mut input_raw)?;

    let input_res: Result<Vec<u64>, Box<dyn std::error::Error>> = input_raw
        .trim()
        .split_ascii_whitespace()
        .map(|raw_str| raw_str.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|parse_err| parse_err.to_string().into());

    input_res
}

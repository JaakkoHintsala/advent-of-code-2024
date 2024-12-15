use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::usize;

use itertools::Itertools;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root MapChar: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-9-first/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let mut after_rearrage = input.clone();

    let (mut filled_part, mut empty_part): (Vec<_>, Vec<_>) =
        input.iter().partition(|item| *item.1 >= 0);
    empty_part.sort_by(|pair1, pair2| pair1.0.cmp(pair2.0));
    filled_part.sort_by(|pair1, pair2| pair2.0.cmp(pair1.0));

    empty_part
        .iter()
        .zip(filled_part.iter())
        .filter(|(empty_part_pair, filled_part_pair)| empty_part_pair.0 < filled_part_pair.0)
        .for_each(|(empty_part_pair, filled_part_pair)| {
            after_rearrage.insert(*empty_part_pair.0, *filled_part_pair.1);
            after_rearrage.insert(*filled_part_pair.0, -1);
        });

    let res = after_rearrage
        .iter()
        .sorted_by_key(|pair| pair.0)
        .filter(|pair| *pair.1 >= 0)
        .fold(0i64, |agg, pair| {
            agg + ((*pair.1) * i64::try_from(*pair.0).unwrap_or_default())
        });

    println!("checksum: {}", res);
}

fn get_disk_map(input: Vec<(i64, i64)>) -> Result<HashMap<usize, i64>, Box<dyn std::error::Error>> {
    let mut disk_index: usize = 0;
    let mut ret: HashMap<usize, i64> = HashMap::new();

    for (file_id, (file_size, empty_space)) in input.iter().enumerate() {
        for _index_in_file in 0..*file_size {
            ret.insert(disk_index, i64::try_from(file_id)?);
            disk_index += 1;
        }

        for _index_in_empty in 0..*empty_space {
            ret.insert(disk_index, -1);
            disk_index += 1;
        }
    }

    Ok(ret)
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<HashMap<usize, i64>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut input_raw = String::new();
    reader.read_to_string(&mut input_raw)?;

    if input_raw.len() % 2 == 1 {
        input_raw.push('0');
    }

    let res = input_raw
        .chars()
        .step_by(2)
        .zip(input_raw.chars().skip(1).step_by(2))
        .map(|(a, b)| {
            let res: Result<(i64, i64), Box<dyn std::error::Error>> = Ok((
                a.to_digit(10)
                    .ok_or_else(|| {
                        let error: Box<dyn std::error::Error> =
                            (a.to_string() + " is not a digit").into();
                        error
                    })?
                    .into(),
                b.to_digit(10)
                    .ok_or_else(|| {
                        let error: Box<dyn std::error::Error> =
                            (b.to_string() + " is not a digit").into();
                        error
                    })?
                    .into(),
            ));
            return res;
        })
        .collect::<Result<Vec<(i64, i64)>, _>>();

    get_disk_map(res?)
}

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

    let input_path = project_root_path.join(Path::new("day-9-second/input.txt"));

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let (filled_part, empty_part): (Vec<_>, Vec<_>) =
        input.clone().into_iter().partition(|item| item.1 >= 0);

    let mut after_rearrage = input.clone();
    let mut empty_blocks = get_empty_disk_blocks(empty_part);
    let filled_blocks = get_disk_blocks(filled_part);

    // println!("input {:?}\n\n", input);
    // println!("filled_blocks {:?}\n\n", filled_blocks);

    for filled_block in filled_blocks {
        // println!("empty_blocks {:?}\n\n", empty_blocks);
        for empty_block in empty_blocks.iter_mut() {

            if empty_block.0 > filled_block.1 {
                continue;
            }

            let filled_block_size = filled_block.2 + 1 - filled_block.1;
            let empty_block_size = empty_block.1 + 1 - empty_block.0;

            if empty_block_size >= filled_block_size {
                for ind_to_switch in 0..filled_block_size {
                    after_rearrage.insert(filled_block.1 + ind_to_switch, -1);
                    after_rearrage.insert(empty_block.0 + ind_to_switch, filled_block.0);
                }
                *empty_block = (empty_block.0 + filled_block_size, empty_block.1);
                break;
            }
        }
    }

    let res = after_rearrage
        .iter()
        .sorted_by_key(|pair| pair.0)
        .filter(|pair| *pair.1 >= 0)
        .fold(0i64, |agg, pair| {
            agg + ((*pair.1) * i64::try_from(*pair.0).unwrap_or_default())
        });

    println!("checksum: {}", res);
}

fn get_empty_disk_blocks(empty_parts: Vec<(usize, i64)>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    let mut current_block: Vec<(usize, i64)> = vec![];

    empty_parts
        .iter()
        .sorted_by_key(|pair| pair.0)
        .for_each(|pair| {
            // println!("pair {:?}", *pair);
            // println!("last {:?}", current_block.last());
            match current_block.last() {
                Some(last) => {
                    if last.0.abs_diff(pair.0) == 1 {
                        current_block.push(*pair);
                    } else {
                        let add_to_res_opt =
                            current_block
                                .iter()
                                .min_by_key(|p| p.0)
                                .and_then(|&min_ind| {
                                    current_block
                                        .iter()
                                        .copied()
                                        .max_by_key(|p| p.0)
                                        .map(|max_ind| (min_ind.0, max_ind.0))
                                });

                        if let Some(add_to_res) = add_to_res_opt {
                            res.push(add_to_res);
                            current_block = vec![*pair];
                        }
                    }
                }
                None => {
                    current_block.push(*pair);
                }
            };
        });

    let add_to_res_opt = current_block
        .iter()
        .min_by_key(|p| p.0)
        .and_then(|&min_ind| {
            current_block
                .iter()
                .copied()
                .max_by_key(|p| p.0)
                .map(|max_ind| (min_ind.0, max_ind.0))
        });
    if let Some(add_to_res) = add_to_res_opt {
        res.push(add_to_res);
    }

    res.sort_by_key(|item| item.0);
    res
}

fn get_disk_blocks(non_empty_parts: Vec<(usize, i64)>) -> Vec<(i64, usize, usize)> {
    let res: Vec<(i64, usize, usize)> = non_empty_parts
        .iter()
        .into_group_map_by(|(_index, file_id)| *file_id)
        .iter()
        .flat_map(|(file_id, index_file_id_pairs)| {
            index_file_id_pairs
                .iter()
                .max_by(|first, sec| first.0.cmp(&sec.0))
                .and_then(|max_ind| {
                    index_file_id_pairs
                        .iter()
                        .min_by(|first, sec| first.0.cmp(&sec.0))
                        .map(|min_ind| {
                            (
                                *file_id, min_ind.0,
                                max_ind.0,
                                // index_file_id_pairs.iter().map(|&&x| x).collect(),
                            )
                        })
                })
        })
        .sorted_by_key(|block| -block.0)
        .collect::<Vec<_>>();
    res
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

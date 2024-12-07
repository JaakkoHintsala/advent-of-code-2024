use std::collections::{HashMap, HashSet, hash_set};
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::iter;
use std::num::ParseIntError;
use std::path::Path;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root dir: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("day-5-first/input.txt"));

    let (input_rules, input_updates) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let rule_map = build_rule_map(input_rules.clone());
    let valid_updates = input_updates
        .iter()
        .filter(|update_vec| {
            let valid = validate(update_vec.to_vec(), rule_map.clone());
            // println!("{}", valid);

            valid
        })
        .collect::<Vec<_>>();


    println!(
        "Middle sum: {:?}",
        valid_updates
            .iter()
            .flat_map(|v| v.get(v.len() / 2))
            .sum::<i64>()
    );
}

fn validate(update_inputs: Vec<i64>, rule_map: HashMap<i64, HashSet<i64>>) -> bool {
    for (ind, val) in update_inputs.iter().enumerate() {
        let (before_part, _after_part) = update_inputs.split_at(ind + 1);

        let rule_set_opt = rule_map.get(val);
        let invalid = rule_set_opt.map(|rule_set| {
            before_part
                .iter()
                .any(|val_in_before_part| rule_set.contains(val_in_before_part))
        });

        if invalid.unwrap_or_else(|| false) {
            return false;
        }
    }

    return true;
}

fn build_rule_map(rules: Vec<[i64; 2]>) -> HashMap<i64, HashSet<i64>> {
    let mut ret: HashMap<i64, HashSet<i64>> = HashMap::new();
    for [before, after] in rules {
        ret.entry(before)
            .and_modify(|after_vals| {
                after_vals.insert(after);
            })
            .or_insert(HashSet::from([after]));
    }
    ret
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<(Vec<[i64; 2]>, Vec<Vec<i64>>), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut input_rules = vec![];
    let mut input_updates = vec![];

    for line_res in reader.lines() {
        let line = line_res?;
        if line.contains("|") {
            input_rules.push(line);
        } else if !line.is_empty() {
            input_updates.push(line);
        }
    }

    let parsed_rules = input_rules
        .iter()
        .map(|rule_str| {
            let rule: Result<[i64; 2], Box<dyn std::error::Error>> = rule_str
                .split('|')
                .map(|string| string.parse::<i64>().map_err(|err| err.to_string().into()))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|int_vec| {
                    let arr: Result<[i64; 2], _> = int_vec
                        .try_into()
                        .map_err(|_| "Failed to parse one of the rules".into());
                    arr
                });
            rule
        })
        .collect::<Result<Vec<_>, _>>();

    let parsed_updates = input_updates
        .iter()
        .map(|rule_str| {
            let rule: Result<_, Box<dyn std::error::Error>> = rule_str
                .split(',')
                .map(|string| string.parse::<i64>().map_err(|err| err.to_string().into()))
                .collect::<Result<Vec<_>, _>>();
            rule
        })
        .collect::<Result<Vec<_>, _>>();

    Ok((parsed_rules?, parsed_updates?))
}

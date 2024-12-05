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

    let input_path = project_root_path.join(Path::new("day-2-first/input.txt"));

    // print!("{}", input_path_str);

    let input = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };

    let safe_count = input.iter().filter(|report| is_safe(
      report.to_vec()  
    )).collect::<Vec<_>>().len();

    println!("{}", safe_count);
}
#[derive(Debug)]
enum ReportType {
    Desc,
    Asc,
    Unset,
}

fn is_safe(report: Vec<i32>) -> bool {
    if report.len() == 0 {
        return true;
    }

    let folded =  report
        .windows(2)
        .fold((ReportType::Unset, true), |aggre, window| {
            let diff = window[1] - window[0];

            if diff.abs() > 3 {
                return (aggre.0, false);
            }

            let ret = match aggre.0 {
                ReportType::Desc => {
                    (ReportType::Desc, if diff < 0 { aggre.1 } else { false })
                }
                ReportType::Asc => {
                    (ReportType::Asc, if diff > 0 { aggre.1 } else { false })
                }
                ReportType::Unset => {
                    (
                        if diff < 0 {
                            ReportType::Desc
                        } else {
                            ReportType::Asc
                        },
                        if diff != 0 { aggre.1 } else { false },
                    )
                }
            };

            return ret;
        });

        // println!("{:?} : {:?}", report, folded);

    folded.1
}

fn read_and_process_input(file_path: &Path) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut input: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        input.push(numbers);
    }
    Ok(input)
}

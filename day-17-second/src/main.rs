use std::cell::Cell;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::rc::Rc;
use std::usize;
use std::{fs::File, path::Path};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let project_root_path = match project_root::get_project_root() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't locate project root: {}", error);
            std::process::exit(1);
        }
    };

    let input_path = project_root_path.join(Path::new("test-inputs/17-2.txt"));

    let (_reg_a, _reg_b, _reg_c, prog) = match read_and_process_input(input_path.as_path()) {
        Ok(valid_input) => valid_input,
        Err(error) => {
            eprintln!("Error while reading input: {}", error);
            std::process::exit(1);
        }
    };
    let desired_output = prog.iter().rev().copied().collect_vec();
    let possible_reg_a_vals = backtrack(vec![0], &prog, &desired_output);

    dbg!(&possible_reg_a_vals);

    let res = possible_reg_a_vals.iter().min();

    println!("Register A value: {:?}", res);
}

fn backtrack(reg_a_after: Vec<i64>, prog: &Vec<i64>, desired_output: &[i64]) -> Vec<i64> {
    let target_output = match desired_output.first() {
        Some(target) => *target,
        None => return reg_a_after,
    };

    let reg_a_vals_before = reg_a_after
        .iter()
        .flat_map(|after_val| get_possible_reg_a_vals_before(*after_val))
        .filter(|possible_before_val| {
            run_one_iteration(*possible_before_val, prog)
                .unwrap_or(-1)
                .eq(&target_output)
        })
        .collect_vec();

    backtrack(reg_a_vals_before, prog, &desired_output[1..])
}

fn get_possible_reg_a_vals_before(reg_a_after: i64) -> Vec<i64> {
    let denom = 2_i64.pow(3); // hardcoded based on input
    let low_val = reg_a_after * denom;

    let mut ret = vec![];
    for value in low_val.max(1)..(low_val + denom) {
        ret.push(value);
    }
    ret
}

fn run_one_iteration(reg_a: i64, prog: &Vec<i64>) -> Option<i64> {
    let register_a = Rc::new(Cell::new(reg_a));
    let register_b = Rc::new(Cell::new(0));
    let register_c: Rc<Cell<i64>> = Rc::new(Cell::new(0));
    let instruction_pointer: Rc<Cell<i64>> = Rc::new(Cell::new(0i64));
    let program: &Vec<i64> = &prog;
    let adv = make_div_op(
        instruction_pointer.clone(),
        register_a.clone(),
        register_a.clone(),
    );

    let bdv = make_div_op(
        instruction_pointer.clone(),
        register_a.clone(),
        register_b.clone(),
    );
    let cdv = make_div_op(
        instruction_pointer.clone(),
        register_a.clone(),
        register_c.clone(),
    );

    let bxl = make_xor_op(instruction_pointer.clone(), register_b.clone());
    let bxc = make_xor_op(instruction_pointer.clone(), register_b.clone());
    let out = make_out_op(instruction_pointer.clone());
    let bst = make_bst_op(instruction_pointer.clone(), register_b.clone());
    let jnz = make_jump_op(instruction_pointer.clone(), register_a.clone());

    let get_combo_operand =
        make_combo_operand_getter(register_a.clone(), register_b.clone(), register_c.clone());

    while let Some((opcode, operand)) =
        get_next_opcode_and_operand(instruction_pointer.clone(), program)
    {
        match opcode {
            0 => adv(get_combo_operand(operand)),
            1 => bxl(operand),
            2 => bst(get_combo_operand(operand)),
            3 => jnz(operand),
            4 => bxc(register_c.get()),
            // my input has only one out opcode
            5 => return Some(out(get_combo_operand(operand))), 
            6 => bdv(get_combo_operand(operand)),
            7 => cdv(get_combo_operand(operand)),
            _ => panic!("illegal opcode"),
        }

        // dbg!(opcode);
        // dbg!(operand);
        // dbg!(instruction_pointer.get());
        // dbg!(register_a.get());
        // dbg!(register_b.get());
        // dbg!(register_c.get());
        // dbg!(output.borrow());
        // print!("\n\n");
    }
    None
}

fn get_next_opcode_and_operand(
    instruction_pointer: Rc<Cell<i64>>,
    program: &Vec<i64>,
) -> Option<(i64, i64)> {
    let index: usize = instruction_pointer.get().try_into().ok()?;
    let opcode = program.get(index)?;
    let operand = program.get(index + 1)?;
    Some((*opcode, *operand))
}

fn make_combo_operand_getter(
    register_a: Rc<Cell<i64>>,
    register_b: Rc<Cell<i64>>,
    register_c: Rc<Cell<i64>>,
) -> impl Fn(i64) -> i64 {
    return move |operand| match operand {
        0..=3 => operand,
        4 => register_a.get(),
        5 => register_b.get(),
        6 => register_c.get(),
        _ => panic!("illegal combo operand"),
    };
}

fn make_div_op(
    instruction_pointer: Rc<Cell<i64>>,
    numerator_reg: Rc<Cell<i64>>,
    target_reg: Rc<Cell<i64>>,
) -> impl Fn(i64) {
    return move |operand| {
        let denom: Result<u32, _> = operand.try_into();
        let new_val = match denom {
            Ok(rhs) => numerator_reg.get() / (2i64.pow(rhs)),
            Err(_) => panic!("Cannot convert {} into u32", operand),
        };
        target_reg.set(new_val);
        instruction_pointer.set(instruction_pointer.get() + 2);
    };
}

fn make_xor_op(instruction_pointer: Rc<Cell<i64>>, target_reg: Rc<Cell<i64>>) -> impl Fn(i64) {
    return move |operand| {
        let new_val = target_reg.get() ^ operand;
        target_reg.set(new_val);
        instruction_pointer.set(instruction_pointer.get() + 2);
    };
}

fn make_out_op(instruction_pointer: Rc<Cell<i64>>) -> impl Fn(i64) -> i64 {
    return move |operand| {
        let out_val = operand % 8;
        instruction_pointer.set(instruction_pointer.get() + 2);
        out_val
    };
}

fn make_bst_op(instruction_pointer: Rc<Cell<i64>>, target_reg: Rc<Cell<i64>>) -> impl Fn(i64) {
    return move |operand| {
        let out_val = operand % 8;
        target_reg.set(out_val);
        instruction_pointer.set(instruction_pointer.get() + 2);
    };
}

fn make_jump_op(instruction_pointer: Rc<Cell<i64>>, _toggle_reg: Rc<Cell<i64>>) -> impl Fn(i64) {
    return move |_operand| {
        // if toggle_reg.get() != 0 {
        //     instruction_pointer.set(operand);
        // } else {
        instruction_pointer.set(instruction_pointer.get() + 2);
        // }s
    };
}

fn read_and_process_input(
    file_path: &Path,
) -> Result<(i64, i64, i64, Vec<i64>), Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut raw_input = String::new();
    reader.read_to_string(&mut raw_input)?;
    parse_regex(&raw_input).map_err(|error| error.to_string().into())
}

fn parse_regex(input: &str) -> Result<(i64, i64, i64, Vec<i64>), ParseIntError> {
    let re = Regex::new(
        r"Register A: (?<register_a>\d+)\nRegister B: (?<register_b>\d+)\nRegister C: (?<register_c>\d+)\n\nProgram: (?<program>[\d,]+)"
    ).unwrap();

    if let Some(captures) = re.captures(&input) {
        let register_a = captures
            .name("register_a")
            .unwrap()
            .as_str()
            .parse::<i64>()?;
        let register_b = captures
            .name("register_b")
            .unwrap()
            .as_str()
            .parse::<i64>()?;
        let register_c = captures
            .name("register_c")
            .unwrap()
            .as_str()
            .parse::<i64>()?;
        let program = captures
            .name("program")
            .unwrap()
            .as_str()
            .split(',')
            .map(|stringi| stringi.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;

        println!("Register A: {}", register_a);
        println!("Register B: {}", register_b);
        println!("Register C: {}", register_c);
        println!("Program: {:?}", program);

        return Ok((register_a, register_b, register_c, program));
    }

    panic!("Input did not match regex.");
}

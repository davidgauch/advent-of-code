extern crate core;

use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("aoc05/input.txt").unwrap();

    let (stack_input, procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stack_input);

    print_stacks(&stacks);

    let answer_1 = process_operation_part_1(stacks.clone(), procedure);
    print_output(&answer_1);

    let answer_2 = process_operation_part_2(stacks, procedure);
    print_output(&answer_2);
}

fn print_output(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!();
}

fn process_operation_part_1(mut stacks: Vec<Vec<char>>, procedure: &str) -> Vec<Vec<char>> {
    let regex: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for operation in procedure.lines() {
        let caps = regex.captures(operation).unwrap();
        let op_size = caps[1].parse::<usize>().unwrap();
        let from_stack = caps[2].parse::<usize>().unwrap() - 1;
        let to_stack = caps[3].parse::<usize>().unwrap() - 1;

        let split_index = stacks[from_stack].len() - op_size;
        let temp = stacks[from_stack].split_off(split_index);
        assert_eq!(temp.len(), op_size);
        stacks[to_stack].extend(temp.into_iter().rev());
    }
    stacks
}

fn process_operation_part_2(mut stacks: Vec<Vec<char>>, procedure: &str) -> Vec<Vec<char>> {
    let regex: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for operation in procedure.lines() {
        let caps = regex.captures(operation).unwrap();
        let op_size = caps[1].parse::<usize>().unwrap();
        let from_stack = caps[2].parse::<usize>().unwrap() - 1;
        let to_stack = caps[3].parse::<usize>().unwrap() - 1;

        let split_index = stacks[from_stack].len() - op_size;
        let temp = stacks[from_stack].split_off(split_index);
        assert_eq!(temp.len(), op_size);
        stacks[to_stack].extend(temp.into_iter());
    }
    stacks
}

fn parse_stacks(stack_input: &str) -> Vec<Vec<char>> {
    let stack_input = stack_input.lines().collect::<Vec<_>>();

    // assert each line is the same length
    let _ = stack_input.iter().map(|&s| s.len()).reduce(|a, b| {
        assert_eq!(a, b);
        a
    });

    let (&stack_nums, stack_input) = stack_input.split_last().unwrap();
    let stack_nums = stack_nums.split_whitespace().collect::<Vec<_>>();

    //assert sane labels
    for (i, &stack_label) in stack_nums.iter().enumerate() {
        assert_eq!((i + 1).to_string(), stack_label);
    }

    let mut stacks = vec![Vec::new(); stack_nums.len()];
    for &layer in stack_input.into_iter().rev() {
        for (position, container) in layer.as_bytes().chunks(4).enumerate() {
            match container.len() {
                3 | 4 => {}
                x => panic!("illegal chunk size {x}"),
            }
            if container[0] == b'[' && container[2] == b']' {
                stacks[position].push(char::from(container[1]))
            }
        }
    }

    stacks
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    let height = stacks.iter().map(Vec::len).max().unwrap_or(0);

    for i in (0..height).rev() {
        for stack in stacks {
            if let Some(item) = stack.get(i) {
                print!("[{item}] ");
            } else {
                print!("    ");
            }
        }
        println!();
    }
    for i in 1..=stacks.len() {
        print!(" {i}  ");
    }
    println!()
}

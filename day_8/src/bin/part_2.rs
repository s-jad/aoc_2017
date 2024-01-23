use itertools::{partition, Itertools};
use std::{collections::HashMap, time::Instant};

fn parse_instruction<'a, 'b>(
    mut registers: HashMap<&'a str, i32>,
    line: &'a str,
    highest: &'b mut i32,
) -> (&'b mut i32, HashMap<&'a str, i32>) {
    let parts = line.split_whitespace().collect_vec();
    let (r1, r2) = (parts[0], parts[4]);

    match (registers.contains_key(r1), registers.contains_key(r2)) {
        (true, false) => {
            registers.insert(r2, 0);
        }
        (false, true) => {
            registers.insert(r1, 0);
        }
        (false, false) => {
            registers.insert(r1, 0);
            registers.insert(r2, 0);
        }
        _ => {}
    }
    let (r, p) = (
        registers.get(r2).unwrap(),
        &parts[6].parse::<i32>().expect("parts[6] is num)"),
    );

    let cond = match parts[5] {
        "<" => r < p,
        ">" => r > p,
        "==" => r == p,
        "!=" => r != p,
        "<=" => r <= p,
        ">=" => r >= p,
        _ => unreachable!(),
    };

    if cond {
        let ins = match parts[1] {
            "inc" => parts[2].parse::<i32>().unwrap(),
            "dec" => -(parts[2].parse::<i32>().unwrap()),
            _ => unreachable!(),
        };

        let num = registers.get_mut(&r1).unwrap();
        *num += ins;
    }

    let new_highest = *registers.values().max().unwrap();

    if new_highest > *highest {
        *highest = new_highest
    }

    (highest, registers)
}

fn process(input: &str) -> i32 {
    let (highest, _) = input
        .lines()
        .fold((0, HashMap::new()), |(mut highest, registers), line| {
            let (new_highest, new_registers) = parse_instruction(registers, line, &mut highest);
            (*new_highest, new_registers)
        });

    highest
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}

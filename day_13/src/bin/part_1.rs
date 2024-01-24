use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Layer {
    layer: usize,
    depth: usize,
    scanner_pos: usize,
    current_dir: i32,
}

fn process(input: &str) -> usize {
    let mut scanners = input
        .lines()
        .map(|l| {
            let (layer, depth) = l
                .split_terminator(&[' ', '\n', ':'][..])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(_, _)>()
                .unwrap();
            Layer {
                layer,
                depth,
                scanner_pos: 0usize,
                current_dir: 1i32,
            }
        })
        .collect_vec();

    let final_layer = scanners.iter().max_by_key(|l| l.layer).unwrap().layer;
    let mut packet = 0;
    let mut severity = 0;

    while packet <= final_layer {
        let current_scanner = scanners.iter().find(|l| l.layer == packet);

        if current_scanner.is_some() {
            let cs = current_scanner.unwrap();

            if cs.scanner_pos == 0 {
                severity += cs.layer * cs.depth;
            }
        }

        for r in scanners.iter_mut() {
            r.scanner_pos = (r.scanner_pos as i32 + r.current_dir) as usize;
            if r.scanner_pos == 0 {
                r.current_dir = 1;
            } else if r.scanner_pos == r.depth - 1 {
                r.current_dir = -1;
            }
        }

        packet += 1;
    }

    severity
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

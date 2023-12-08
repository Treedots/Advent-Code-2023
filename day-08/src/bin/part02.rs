use console::Term;
use core::panic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
fn main() {
    let data = include_str!("../../../inputs/day08.txt");
    let result = logic(data);
    println!("{result}");
}
const END_POINT: &str = "ZZZ";

fn logic(data: &str) -> i64 {
    let data = data.replace("(", "").replace(")", "");
    let mut instruction = "";
    let mut has: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_points: Vec<&str> = Vec::new();
    let mut points: Vec<&str> = Vec::new();
    for (i, v) in data.lines().enumerate() {
        if i == 0 {
            instruction = v;
        } else if v.is_empty() {
        } else {
            let (main, (left, right)): (&str, (&str, &str)) = v
                .split_once("=")
                .map(|(a, b)| {
                    (
                        a.trim(),
                        b.trim()
                            .split_once(",")
                            .map(|(a, b)| (a.trim(), b.trim()))
                            .unwrap(),
                    )
                })
                .unwrap();
            if main.ends_with("A") {
                start_points.push(main);
            }
            points.push(main);
            has.insert(main, (left, right));
        }
    }
    let mut new_hash: HashMap<&str, String> = HashMap::new();

    for i in &points {
        let t = loop_process(i, instruction, has.clone());
        new_hash.insert(i, t);
    }
    let mut total = 1;
    let mut no_of_z = 0;
    println!("{total}");
    loop {
        let mut temp_hash: HashMap<&str, String> = HashMap::new();
        for i in points.iter() {
            let mut temp = i;
            for _ in 0..2 {
                let t = new_hash.get(temp).unwrap();
                if i.ends_with("A") && t.ends_with("z") {
                    no_of_z += 1;
                }
                temp_hash.insert(i, t.to_string());
            }
        }
        total += 1;
        if no_of_z == start_points.len() {
            break;
        } else {
            new_hash = temp_hash.clone();
            temp_hash.clear();
        }
    }
    total as i64
}
fn loop_process(start_point: &str, instruction: &str, map: HashMap<&str, (&str, &str)>) -> String {
    let mut current = start_point;
    for v in instruction.chars() {
        let (left, right) = map.get(current).unwrap();
        current = match v {
            'L' => left,
            _ => right,
        };
    }
    current.to_owned()
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day08-test-part02.txt");
        assert_eq!(logic(data), 6);
    }
}

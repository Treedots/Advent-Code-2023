use console::Term;
use core::panic;
use num::Integer;
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

    println!("{new_hash:?}");
    let t: Vec<i64> = start_points
        .iter()
        .map(|f| loop_get_z(f, new_hash.clone()))
        .collect();
    let (_, temp) = t.split_at(0);
    let t = lcma(temp);
    let result =  t * instruction.len() as i64;
    result
}

fn lcma(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcma(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn loop_get_z(start: &str, map: HashMap<&str, String>) -> i64 {
    let mut current = start;
    let mut count = 0;
    loop {
        count += 1;
        let temp = map.get(current).unwrap();
        if temp.ends_with("Z") {
            break;
        } else {
            current = temp;
        }
    }
    count
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

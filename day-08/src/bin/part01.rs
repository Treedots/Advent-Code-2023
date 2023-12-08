use core::panic;
use std::collections::HashMap;

fn main() {
    let data = include_str!("../../../inputs/day08.txt");
    let result = logic(data);
    println!("{result}");
}
const END_POINT: &str = "ZZZ";

fn logic(data: &str) -> i32 {
    let data = data.replace("(", "").replace(")", "");
    let mut instruction = "";
    let mut has: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_point = "AAA";
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
            if start_point.is_empty() {
                start_point = main.clone();
            }
            has.insert(main, (left, right));
        }
    }
    let mut count = 0;
    let mut total = 0;
    let mut a: String;
    let mut b;
    while start_point != END_POINT {
        (a, b) = loop_process(start_point, instruction, has.clone());
        total += b;
        start_point = &a;
        if count > 100 {
            panic!("Ran more that 100 loops")
        } else {
            count += 1;
        }
    }
    total
}

fn loop_process(
    start_point: &str,
    instruction: &str,
    map: HashMap<&str, (&str, &str)>,
) -> (String, i32) {
    let mut current = start_point;
    let mut count = 0;
    for v in instruction.chars() {
        let (left, right) = map.get(current).unwrap();
        current = match v {
            'L' => left,
            _ => right,
        };
        count += 1;
        if current == END_POINT {
            break;
        }
    }
    (current.to_owned(), count)
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day08-test.txt");
        assert_eq!(logic(data), 2);
    }

    #[test]
    fn check_logic_2() {
        let data = include_str!("../../../inputs/day08-test-2.txt");
        assert_eq!(logic(data), 6);
    }
}

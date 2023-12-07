use std::{collections::HashMap, usize};
fn main() {
    let data = include_str!("../../../inputs/day07.txt");
    let result = logic(data);
    println!("Resutl: {result}");
}

fn logic(data: &str) -> i32 {
    //Process Data
    let mut collection: Vec<Vec<(&str, String, i32)>> = vec![Vec::new(); 7];
    let mut camel_cards: Vec<(&str, String, i32)> = Vec::new();
    for i in data.lines() {
        let (a, header, body): (&str, String, i32) = i
            .split_once(" ")
            .map(|(a, b)| {
                (
                    a,
                    a.trim()
                        .replace("A", "D")
                        .replace("K", "C")
                        .replace("Q", "B")
                        .replace("J", "0")
                        .replace("T", "A"),
                    b.trim().parse::<i32>().unwrap(),
                )
            })
            .unwrap();
        camel_cards.push((a, header, body));
    }
    //
    for (a, j, k) in camel_cards {
        let hand = check_hand(&j);
        collection[hand].push((a, j, k));
    }
    for c in &mut collection {
        c.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    }
    let mut total = 0;
    let mut counter = 1;
    for (_i,c) in collection.iter().enumerate() {
        c.iter().for_each(|(_k, _a, v)| {
            //println!("{counter}:{_k}:{_a} - {i}");
            total += counter as i32 * v;
            counter += 1;
        })
    }
    total
}

fn check_hand(data: &str) -> usize {
    let mut d: HashMap<String, i32> = HashMap::new();
    let mut joker = 0;
    data.chars().for_each(|f| {
        let g = f.to_string();
        if g == String::from("0") {
            joker += 1;
        }
        if d.contains_key(&g) {
            let t = d.get_mut(&g);
            if let Some(v) = t {
                *v += 1;
            }
        } else {
            d.insert(g, 1);
        }
    });
    let mut dt: Vec<String> = d.iter().map(|(_, v)| v.to_string()).collect();
    dt.sort_by(|a, b| b.cmp(a));
    let binding = dt.join("");
    let result = binding.as_str();
    // 5 6
    // 41 5
    // 32 4
    // 311 3
    // 221 2
    // 2111 1
    // 11111 0
    match (result, joker) {
        ("5", 5) => 6,
        ("5", 0) => 6,
        ("41", 4) => 6,
        ("41", 1) => 6,
        ("41", 0) => 5,
        ("32", 3) => 6,
        ("32", 2) => 6,
        ("32", 0) => 4,
        ("311", 3) => 5,
        ("311", 1) => 5,
        ("311", 0) => 3,
        ("221", 2) => 5,
        ("221", 1) => 4,
        ("221", 0) => 2,
        ("2111", 2) => 3, //1
        ("2111", 1) => 3,
        ("2111", 0) => 1,
        ("11111", 1) => 1,
        ("11111", 0) => 0,
        _ => panic!("Failed to process"),
    }
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day07-test.txt");
        assert_eq!(logic(data), 5905);
    }
}

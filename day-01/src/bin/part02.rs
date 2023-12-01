fn main() {
    let data = include_str!("../../../inputs/day01.txt");
    let t = logic(data);
    println!("{t}");
}

fn logic(data: &str) -> u32 {
    let mut total = 0;
    for i in data.lines() {
        let temp = replace_spelled(i);
        let v: Vec<u32> = temp.chars().filter_map(|a| a.to_digit(10)).collect();
        if v.len() == 1 {
            println!("{0}{0}", v[0]);
            total += v[0] * 11;
        } else {
            println!("{}{}", v[0], v[v.len() - 1]);
            total += v[0] * 10 + v[v.len() - 1];
        }
    }
    total
}

fn replace_spelled(i: &str) -> String {
    let mut tmp = String::from(i);
    let replacements: Vec<(&str, &str)> = vec![
        ("one", "o-1-e"),
        ("two", "t-2-o"),
        ("three", "t-3-e"),
        ("four", "f-4-r"),
        ("five", "f-5-e"),
        ("six", "s-6-x"),
        ("seven", "s-7-n"),
        ("eight", "e-8-t"),
        ("nine", "n-9-e"),
    ];
    for (x, y) in replacements.iter() {
        tmp = tmp.replace(x, y);
    }
    println!("{tmp}");
    tmp
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn checkvalue() {
        let data = include_str!("../../../inputs/day01-test.txt");
        let total = logic(data);
        assert_eq!(total, 281);
    }
}

fn main() {
    let data = include_str!("../../../inputs/day06.txt");
    let result = logic(data);
    println!("{result}");
}

fn logic(data: &str) -> i32 {
    let mut time_array: Vec<i32> = Vec::new();
    let mut dist_array: Vec<i32> = Vec::new();
    data.lines().enumerate().for_each(|(i, f)| {
        let (_, body): (&str, Vec<i32>) = f
            .split_once(":")
            .map(|(a, b)| {
                (
                    a.trim(),
                    b.trim()
                        .split(" ")
                        .filter(|f| !f.is_empty())
                        .map(|f| f.trim().parse::<i32>().unwrap())
                        .collect(),
                )
            })
            .unwrap();
        if i == 0 {
            time_array = body;
        } else {
            dist_array = body;
        }
    });
    let t = (0..time_array.len())
        .map(|i| {
            let time_limit = time_array[i];
            let distance = dist_array[i];
            (0..time_array[i])
                .map(move |k| {
                    let speed = k.clone();
                    let time_remaining = time_limit - speed;
                    if speed * time_remaining > distance {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .reduce(|a, b| a * b)
        .unwrap();
    t
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day06-test.txt");
        assert_eq!(logic(data), 288);
    }
}

fn main() {
    let data = include_str!("../../../inputs/day06.txt");
    let result = logic(data);
    println!("{result}");
}

fn logic(data: &str) -> i64 {
    let mut time_array: i64 = 0;
    let mut dist_array: i64 = 0;
    data.lines().enumerate().for_each(|(i, f)| {
        let (_, body): (&str, i64) = f
            .split_once(":")
            .map(|(a, b)| {
                (
                    a.trim(),
                    b.trim().replace(" ", "").parse::<i64>().unwrap()
                        ,
                )
            })
            .unwrap();
        if i == 0 {
            time_array = body;
        } else {
            dist_array = body;
        }
    });
    let time_limit = time_array;
    let distance = dist_array;
            (0..time_array)
                .map(move |k| {
                    let speed = k.clone();
                    let time_remaining = time_limit - speed;
                    if speed * time_remaining > distance {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day06-test.txt");
        assert_eq!(logic(data), 71503);
    }
}

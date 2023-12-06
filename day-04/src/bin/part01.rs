fn main(){
    let data =include_str!("../../../inputs/day04.txt");
    let result = logic(data);
    println!("Result: {result}");
}
#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic(){
        let data = include_str!("../../../inputs/day04-test.txt");
        assert_eq!(logic(data),13);
    }
}
fn logic(data:&str)->i32{
    let mut total = 0;
    for i in data.lines(){
        let (header,body) = i.split_once(":").map(|(a,b)|(a.trim(),b.trim())).unwrap();
        println!("{header}\n{body}");
        let (card,winnings):(Vec<i32>,Vec<i32>) =  i.split_once("|").map(|(a,b)|(a.trim().split(" ").map(|f| f.parse::<i32>().unwrap_or(-99)).collect(),b.trim().split(" ").map(|f| f.parse::<i32>().unwrap_or(-999) ).collect())).unwrap();
        let mut score = 0;
        for i in card{
            if winnings.contains(&i){
                if score == 0{
                    score = 1;
                }
                else{
                    score *=2;
                }
            }
        }
        total += score;
    }

    total
}

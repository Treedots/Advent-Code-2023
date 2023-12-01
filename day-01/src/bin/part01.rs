fn main(){
    let data = include_str!("../../../inputs/day01.txt");
    let mut total:u32 = 0;
    data.lines().for_each(|f|{
        let t: Vec<_> = f.chars().filter_map(|a| a.to_digit(10)).collect();
        let length = t.len();
        if length ==1{
            total += t[0]*10 + t[0];
        }
        else{
            total += t[0]*10 + t[length-1];
        }
    });
    println!("{total}");
}

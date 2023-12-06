fn main(){
    let data =include_str!("../../../input/day04.txt");
    let result = logic(data);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic(){
        let data = include_str!("../../../input/day04-test.txt");
        assert_eq!(logic(data),30);
    }
}

fn logic(data:&str)->i32{
    let no_of_cards = data.lines().count();
    let mut new_instances:Vec<i32> = vec![1;no_of_cards];
    for (index,i) in data.lines().enumerate(){
        let (header,body) = i.split_once(":").map(|(a,b)|(a.trim(),b.trim())).unwrap();
        println!("{header}\n{body}");
        let (card,winnings):(Vec<i32>,Vec<i32>) =  i.split_once("|").map(|(a,b)|(a.trim().split(" ").map(|f| f.parse::<i32>().unwrap_or(-99)).collect(),b.trim().split(" ").map(|f| f.parse::<i32>().unwrap_or(-999) ).collect())).unwrap();
        let mut index_modifer = 1;
        for i in card{
            if winnings.contains(&i){
                new_instances[index +index_modifer as usize] += new_instances[index];
                index_modifer += 1;
            }
        }
    }
    new_instances.into_iter().sum()
}

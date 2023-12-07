use std::{collections::HashMap, usize};
fn main(){
    let data =  include_str!("../../../inputs/day07.txt");
    let result = logic(data);
    println!("Resutl: {result}");
}

fn logic(data:&str)->i32{
    //Process Data
    let mut collection: Vec<Vec<(&str,String,i32)>> = vec![Vec::new();7];
    let mut camel_cards: Vec<(&str,String,i32)> = Vec::new();
    for i in data.lines(){
        let (a,header,body):(&str,String,i32) =i.split_once(" ").map(|(a,b)|(a,a.trim().replace("A", "E").replace("K", "D").replace("Q", "C").replace("J", "B").replace("T", "A"),b.trim().parse::<i32>().unwrap())).unwrap();
        camel_cards.push((a,header,body));
    }
    //
    for (a,j,k) in camel_cards{
         let hand = check_hand(&j);
         collection[hand].push((a,j,k));
    }
    for c in &mut collection{
        c.sort_by(|(_,a,_),(_,b,_)|a.cmp(b));
    }
    let mut total = 0;
    let mut counter = 1;
    for c in collection{
        c.iter().for_each(|(k,a,v)|{
            total += counter as i32 * v;
            counter +=1;
        })
    }
    total
}

fn check_hand(data:&str)->usize{
    let mut d:HashMap<String,i32>=HashMap::new();
    data.chars().for_each(|f|{
        let g = f.to_string();
        if d.contains_key(&g){
            let t = d.get_mut(&g);
            if let Some(v) = t{
                *v += 1;
            }
        }
        else{
            d.insert(g, 1);
        }
    });
    let mut dt:Vec<String> =d.iter().map(|(_,v)|{
        v.to_string()
    }
    ).collect();
    dt.sort_by(|a,b|b.cmp(a));
    let binding = dt.join("");
    let result = binding.as_str();
    match result{
        "5" => 6,
        "41"=> 5,
        "32"=> 4,
        "311"=>3,
        "221"=>2,
        "2111"=>1,
        "11111"=>0,
        _=>0
    }
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day07-test.txt");
        assert_eq!(logic(data), 6440);
    }
}

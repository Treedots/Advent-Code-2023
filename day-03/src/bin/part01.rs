use std::{collections::HashMap, fs};

fn main(){
    let data = include_str!("../../../inputs/day03.txt");
    let _result = logic(data);    
}
#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic(){
        let data = include_str!("../../../inputs/day03-test.txt");
        assert_eq!(logic(data),4361);   
    }
}
fn logic(i:&str)->i32{
    let mut data_grid:HashMap<String,usize> = HashMap::new();
    let mut unique_no:Vec<String> = Vec::new(); 
    let mut specials: Vec<(i32,i32)> = Vec::new();
    for (index_y,j) in i.lines().enumerate(){
        let mut temp = String::new();
        for (index_x,k) in j.chars().enumerate(){
            let temp_id = format!("{index_y}||{index_x}");
            match k{
                '.' => {
                    if !(temp.is_empty()){
                    unique_no.push(temp.to_string());
                    temp = String::new();
                    }
                },
                '0'..='9' =>{
                    temp.push(k);
                    data_grid.insert(temp_id.to_string(), unique_no.len());
                },
                _ => {
                    if !(temp.is_empty()){
                        unique_no.push(temp.to_string());
                    }
                    specials.push((index_x as i32,index_y as i32));
                    temp =String::new();
                }
            }
        }
        if !(temp.is_empty()){
            unique_no.push(temp.to_string());
        }
    }
    println!("{data_grid:?}");
    println!("{specials:?}");
    println!("{unique_no:?}");
    let mut taken:Vec<usize> = Vec::new();
    let mut total = 0;
    let mut result = String::new();
    
    for (x,y) in specials{
        let range_sec = vec![(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)];
        println!("--");
        for (xr,yr) in range_sec.into_iter(){
            println!("{},{}",x+xr,y+yr);
            let index = format!("{}||{}",y+yr,x+xr);
            let temp = data_grid.get(&index);
            if let Some(t) = temp{
                let u = t.clone();
                let no = unique_no.get(u).unwrap();
                if !(taken.contains(t)){
                    result.push_str(format!("\n{x},{y}:{no}").as_str());
                    total += no.parse::<i32>().unwrap();
                    taken.push(t.clone());
                }
            }
        }
    }
    fs::write("temp.text", result).unwrap();
    println!("{total}");
    total
}

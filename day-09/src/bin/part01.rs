use rayon::prelude::*;

fn main(){
    let data =  include_str!("../../../inputs/day09.txt");
    let result =  logic(data);
    println!("{result}");
}
fn logic(data:&str)->i64{
    let data_array: Vec<Vec<i64>> = data.lines().map(|f| f.split(" ").map(|k| k.trim().parse::<i64>().unwrap()).collect()).collect();
    let result = data_array.par_iter().map(|v|{
        let temp = v.clone();
        repeat_this(temp)
    }).sum::<i64>();

    result
}

fn repeat_this(v:Vec<i64>)->i64{
    let mut temp: Vec<i64> = Vec::new();
    let length =  v.len() -1;
    let mut count = 0;
    for i in 0..length{
        let tv = v[i+1]-v[i];
        print!("{tv} ");
        if tv == 0{
            count +=1;
        }
        temp.push(tv);
    }
    println!("");
    if count == temp.len(){
        *v.last().unwrap_or(&0)
    }
    else{
        let temp2 =temp.clone();
        println!("{:?}",temp.last());
        let j = v.last().unwrap_or(&0) + repeat_this(temp2);
        j
    }


}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day09-test.txt");
        assert_eq!(logic(data), 114);
    }

}

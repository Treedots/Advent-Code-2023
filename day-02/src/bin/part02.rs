use std::cmp;

fn main(){
    let data = include_str!("../../../inputs/day02.txt");
    println!("{}",logic(data));
    
}
enum Color {
    Red,
    Green,
    Blue
}
fn match_color(color:&str)->Color{
    match color {
        "green" => Color::Green,
        "red" => Color::Red,
        "blue" => Color::Blue,
        _ => todo!()
    }
}
fn logic(i:&str)->i32{
    let mut total = 0;
    for v in i.lines(){
        let (t,k) = v.split_once(":").map(|(k,y)|(k,y.trim())).unwrap();
        let id = t.replace("Game ", "");
        let (mut r,mut g,mut b):(i32,i32,i32) = (0,0,0);
        k.replace(";", ",").split(",").for_each(|f|{
            let (number,color) = f.trim()
            .split_once(" ")
            .map(|(n,c)| (n.trim().parse::<i32>().unwrap(),match_color(c.trim()))).unwrap();
            match color{
                Color::Red => r =cmp::max(r, number),
                Color::Green => g = cmp::max(g, number),
                Color::Blue => b = cmp::max(b, number)
            }
        });
        println!("{id},{r},{g},{b}");
        total += r*g*b;

    }

    total
}

#[cfg(test)]
mod tests {
    use crate::logic;


    #[test]
    fn it_works() {
        let test = include_str!("../../../Inputs/day02-test.txt");
        let data = logic(test);
        assert_eq!(data,2286);
    }
}

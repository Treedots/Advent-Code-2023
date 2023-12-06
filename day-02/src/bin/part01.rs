
#[derive(Debug)]
enum Color {
    Blue,
    Green,
    Red,
}

fn main(){
    let data = include_str!("../../../inputs/day02.txt");
    let data= logic(data);
    let total = check_limit(data, 12, 13, 14);
    println!("result: {total}");
}

fn logic(data:&str)->Vec<(i32,Vec<Vec<(Color,i32)>>)>{
    let mut data_split:Vec<(i32,Vec<Vec<(Color,i32)>>)>= Vec::new();
    for i in data.lines(){
        let temp01:Vec<&str> = i. split(":").collect();
        let game_id = temp01[0].replace("Game ", "");
        let game_data: Vec<Vec<(Color,i32)>> = temp01[1].split(";").map(|f| f.split(",").map(|g| check_color_and_quantity(g.trim())).collect()).collect();
        //println!("{game_id}:{game_data:?}");
        data_split.push((game_id.parse::<i32>().unwrap(),game_data));
    }
    println!("{}",data_split.len());
    data_split
}

fn check_limit(data:Vec<(i32,Vec<Vec<(Color,i32)>>)>,limits_r:i32,limits_g:i32,limits_b:i32)->i32{
    let mut total = 0;
    for (i,t) in data{
        let mut temp_r = 0;
        let mut temp_g: i32 = 0;
        let mut temp_b: i32 = 0;

        for g in t{
            for (c,d) in g{
                match c{
                    Color::Blue => temp_b += d,
                    Color::Green =>temp_g +=d,
                    Color::Red => temp_r += d
                }
                if temp_r > limits_r || temp_g > limits_g || temp_b > limits_b{
                    break;
                }   
            }
            if temp_r > limits_r || temp_g > limits_g || temp_b > limits_b{
                break;
            }
            else{
                temp_r =0;
                temp_g = 0;
                temp_b = 0;
            }
            
        }
        println!("{i},{temp_r},{temp_g},{temp_b}"); 
        if temp_r <= limits_r && temp_g <= limits_g && temp_b <= limits_b{
                total += i;
        }
    }
    total

}



fn check_color_and_quantity(i:&str)->(Color,i32){
    let temp: Vec<&str> =  i.split(" ").collect();
    let value = temp[0].parse::<i32>().unwrap();
    let color = match temp[1] {
        "green" => Color::Green,
        "red" => Color::Red,
        "blue" => Color::Blue,
        _ => panic!("AHHHH")
    };
    (color,value)
}

#[cfg(test)]
mod tests {
    use crate::{logic, check_limit};

    #[test]
    fn it_works() {
        let test = include_str!("../../../inputs/day02-test.txt");
        let data = logic(test);
        let result = check_limit(data, 12, 13, 14);
        assert_eq!(result,8);
    }
}

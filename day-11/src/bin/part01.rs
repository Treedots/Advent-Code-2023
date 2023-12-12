fn main() {
    let data = include_str!("../../../inputs/day11.txt");
    let result = logic(data);
    println!("Result: {}", result);
}

fn logic(data: &str) -> i64 {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<i32> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let no_of_rows = data.lines().count();
    let mut no_of_cols = data.lines().next().unwrap().len();
    for (index, line) in data.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        let mut count = 0;
        for c in line.chars() {
            row.push(c);
            if c == '#' {
                count += 1;
            }
        }
        if count == 0 {
            empty_rows.push(index)
        }
        grid.push(row);
    }
    for i in 0..no_of_cols {
        let mut count = 0;
        for j in 0..no_of_rows {
            if grid[j][i] == '#' {
                count += 1;
            }
        }
        if count == 0 {
            empty_cols.push(i as i32);
        }
    }

    for i in empty_rows {
        grid[i] = vec!['-'; no_of_cols];
    }
    for i in empty_cols {
        for j in 0..no_of_rows {
            if grid[j][i as usize] == '-' {
                grid[j][i as usize] = '|';
            } else {
                grid[j][i as usize] = '|';
            }
        }
    }

    for i in 0..no_of_rows {
        for j in 0..no_of_cols {
            print!("{}", grid[i][j]);
        }
        println!();
    }

    no_of_cols = grid[0].len();
    let mut position_of_galaxy: Vec<(i64, i64)> = Vec::new();
    let mut offset_x = 0;
    let mut offset_y = 0;
    //Change the offset to 1 to get part 1 ans
    const OFFSET: i64 = 1000000-1;
    for i in 0..no_of_rows {
        for j in 0..no_of_cols {
            if grid[i][j] == '#' {
                println!("{} {}", j as i64 + offset_x, i as i64 + offset_y);
                position_of_galaxy.push((j as i64 + offset_x, i as i64 + offset_y).clone());
            } else if grid[i][j] == '-' {
                offset_y += OFFSET;
                break;
            } else if grid[i][j] == '|' {
                offset_x += OFFSET;
            } 
        }
        offset_x = 0;
    }

    // Calculate of distance of each unique pair
    let mut total = 0;
    for i in 0..position_of_galaxy.len() {
        for j in i + 1..position_of_galaxy.len() {
            let temp = distance(position_of_galaxy[i], position_of_galaxy[j]);
            total += temp;
        }
    }
    total
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    let x = (a.0 - b.0).abs();
    let y = (a.1 - b.1).abs();
    x + y
}

#[cfg(test)]
mod tests {
    use crate::logic;
    #[test]
    fn it_works() {
        let data = include_str!("../../../inputs/day11-test.txt");
        assert_eq!(logic(data), 10);
    }
}

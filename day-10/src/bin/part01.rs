use std::{collections::HashMap, error::Error};

fn main() {
    let data = include_str!("../../../inputs/day10.txt");
    let result = logic(data);
    println!("{result}");
}
#[derive(Debug, Clone, Copy)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl DIRECTION {
    fn get_direction(&self) -> Position {
        match self {
            DIRECTION::UP => Position { x: 0, y: 1 },
            DIRECTION::LEFT => Position { x: -1, y: 0 },
            DIRECTION::RIGHT => Position { x: 1, y: 0 },
            DIRECTION::DOWN => Position { x: 0, y: -1 },
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
struct Grid {
    pos: Position,
    direction_a: DIRECTION,
    direction_b: DIRECTION,
    shape: char,
}

impl Grid {
    fn get_direction(&self) -> [(i32, i32); 2] {
        let (x, y) = (self.pos.x, self.pos.y);
        let t = self.direction_a.get_direction();
        let t2 = self.direction_b.get_direction();
        [(x + t.x, y + t.y), (x + t2.x, y + t2.y)]
    }
}

fn match_grid(i: char, x: i32, y: i32) -> Option<Grid> {
    match i {
        '7' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::LEFT,
            direction_b: DIRECTION::DOWN,
            shape: '7',
        }),
        '|' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::UP,
            direction_b: DIRECTION::DOWN,
            shape: '|',
        }),
        '-' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::LEFT,
            direction_b: DIRECTION::RIGHT,
            shape: '-',
        }),
        'L' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::UP,
            direction_b: DIRECTION::RIGHT,
            shape: 'L',
        }),
        'J' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::UP,
            direction_b: DIRECTION::LEFT,
            shape: 'J',
        }),
        'F' => Some(Grid {
            pos: Position { x, y },
            direction_a: DIRECTION::RIGHT,
            direction_b: DIRECTION::DOWN,
            shape: 'F',
        }),
        _ => None,
    }
}

fn logic(data: &str) -> i32 {
    //Process GRID
    let mut start_point: Position = Position { x: 0, y: 0 };
    let mut map: HashMap<(i32, i32), Option<Grid>> = HashMap::new();
    let mut heat_map: HashMap<(i32, i32), i32> = HashMap::new();
    let no_of_lines: i32 = data.lines().count() as i32 - 1;
    data.lines().enumerate().for_each(|(y, v)| {
        v.chars().enumerate().for_each(|(x, vv)| {
            //println!("{x}:{y}");
            if vv == 'S' {
                start_point = Position {
                    x: x as i32,
                    y: no_of_lines - y as i32,
                };
                map.insert((x as i32, no_of_lines - y as i32), None);
                heat_map.insert((x as i32, no_of_lines - y as i32), 0);
            } else {
                let temp = match_grid(vv, x as i32, no_of_lines - y as i32);
                map.insert((x as i32, no_of_lines - y as i32), temp);
                heat_map.insert((x as i32, no_of_lines - y as i32), 0);
            }
        })
    });
    // Get Connecting Pipes
    let mut next_pipes: Vec<Grid> = Vec::new();
    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (x, y) in directions {
        let temp = map.get(&(start_point.x + x, start_point.y + y));
        match temp {
            Some(k) => match k {
                Some(k) => {
                    let temp = k.get_direction();
                    println!("{k:?}");
                    for (x, y) in temp {
                        if x == start_point.x && y == start_point.y {
                            next_pipes.push(k.clone());
                            if let Some(t) = heat_map.get_mut(&(k.pos.x, k.pos.y)) {
                                *t = 1;
                            }
                        }
                    }
                }
                None => {}
            },
            None => {}
        }
    }
    let mut temp_next:Vec<Grid> = Vec::new();
    let mut max_value = 0;
    while next_pipes.len() > 0 {
        for t in next_pipes.iter() {
            let heat_value = &heat_map.get(&(t.pos.x, t.pos.y)).unwrap().clone() + 1;
            let temp = t.get_direction();
            for (xx, yy) in temp {
                if let Some(a) = map.get(&(xx, yy)) {
                    if let Some(b) = a {
                        if let Some(t) = heat_map.get_mut(&(b.pos.x, b.pos.y)) {
                            if t == &mut 0 {
                                *t = heat_value;
                                max_value = max_value.max(heat_value);
                                temp_next.push(b.clone());
                            }
                        }
                    }
                }
            }
        }
        if temp_next.is_empty(){
            break;
        }
        next_pipes =  temp_next.clone();
        temp_next.clear()
    }
    max_value
    

}
#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic() {
        let data = include_str!("../../../inputs/day10-test-01.txt");
        assert_eq!(logic(data), 4);
    }
    #[test]
    fn check_logic_2() {
        let data = include_str!("../../../inputs/day10-test-02.txt");
        assert_eq!(logic(data), 8);
    }
}

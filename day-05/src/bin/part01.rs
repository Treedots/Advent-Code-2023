#[derive(Clone)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl MapType {
    fn next(&mut self) -> MapType {
        match self {
            MapType::Seed => MapType::Soil,
            MapType::Soil => MapType::Fertilizer,
            MapType::Fertilizer => MapType::Water,
            MapType::Water => MapType::Light,
            MapType::Light => MapType::Temperature,
            MapType::Temperature => MapType::Humidity,
            MapType::Humidity => MapType::Location,
            MapType::Location => MapType::Location,
        }
    }
}
struct MapStruct {
    s_range: i64,
    s_range_limit: i64,
    d_range: i64,
}
impl MapStruct {
    fn check_limits(&self, v: i64) -> i64 {
        if v <= self.s_range_limit && v >= self.s_range {
            let diff = v - self.s_range;
            self.d_range + diff
        } else {
            -1
        }
    }
}

fn logic(data: &str) -> i64 {
    // Read Data
    let mut current = MapType::Seed;
    let mut seeds: Vec<(i64, MapType)> = Vec::new();
    let mut map_soil: Vec<MapStruct> = Vec::new();
    let mut map_fertilizer: Vec<MapStruct> = Vec::new();
    let mut map_water: Vec<MapStruct> = Vec::new();
    let mut map_light: Vec<MapStruct> = Vec::new();
    let mut map_temperature: Vec<MapStruct> = Vec::new();
    let mut map_humidity: Vec<MapStruct> = Vec::new();
    let mut map_location: Vec<MapStruct> = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
        } else if line.contains("seeds") {
            //Pull Seeds Data
            let (_, body): (&str, Vec<(i64, MapType)>) = line
                .split_once(":")
                .map(|(a, b)| {
                    (
                        a.trim(),
                        b.trim().split(" ")
                            .map(|v| (v.trim().parse::<i64>().unwrap(), MapType::Seed))
                            .collect(),
                    )
                })
                .unwrap();
            seeds = body;
        } else if line.contains("map") {
            current = current.next();
        } else {
            let v: Vec<i64> = line
                .split(" ")
                .map(|f| f.trim().parse::<i64>().unwrap())
                .collect();
            let destination_range = v[0];
            let source_range = v[1];
            let source_range_limit = v[1] + v[2] - 1;
            match current {
                //Do nothing for Seed
                MapType::Seed => {}
                MapType::Soil => {
                    map_soil.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Fertilizer => {
                    map_fertilizer.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Water => {
                    map_water.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Light => {
                    map_light.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Temperature => {
                    map_temperature.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Humidity => {
                    map_humidity.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
                MapType::Location => {
                    map_location.push(MapStruct {
                        s_range: source_range,
                        s_range_limit: source_range_limit,
                        d_range: destination_range,
                    });
                }
            }
        }
    }
    // Read Data - End
    let data: Vec<_> = seeds
        .iter()
        .map(|(s, _v)| {
            let mut current = s.clone();
            for i in 0..=6 {
                let temp;
                match i {
                    0 => {
                        temp = map_soil
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    1 => {
                        temp = map_fertilizer
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    2 => {
                        temp = map_water
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    3 => {
                        temp = map_light
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    4 => {
                        temp = map_temperature
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    5 => {
                        temp = map_humidity
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                    _ => {
                        temp = map_location
                            .iter()
                            .map(|f| f.check_limits(current.clone()))
                            .max()
                            .unwrap();
                        if temp != -1 {
                            current = temp
                        }
                    }
                }
                //println!("{current}");
            }
            current
        })
        .collect();
    //println!("{data:?}");
    *data.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::logic;

    #[test]
    fn check_logic(){
        let data = include_str!("../../../inputs/day05-test.txt");
        assert_eq!(logic(data),35);
    }
}


fn main() {
    let data = include_str!("../../../inputs/day05.txt");
    let result = logic(data);
    println!("{result}");
}

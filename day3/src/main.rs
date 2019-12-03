use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

fn get_vector(direction: char) -> [i32; 2] {
    match direction {
        'U' => [0, 1],
        'D' => [0, -1],
        'L' => [-1, 0],
        'R' => [1, 0],
        _ => panic!("bad direction")
    }
}

fn get_wire_path(line: &str, visited: Option<&HashMap<[i32; 2], i32>>) -> HashMap<[i32; 2], i32> {
    let mut hs = HashMap::new();
    let tokens = line.split(',');
    let mut coords = [0,0];

    let mut closest = 0xffff;
    let mut least_steps = 0xffffff;

    let mut steps_taken = 0;
    for token in tokens {
        let (direction, steps) = token.split_at(1); 
        let direction = direction.parse::<char>().unwrap();
        let steps = steps.parse::<i32>().unwrap();
        let movement_vector = get_vector(direction);

        for _ in 0..steps {
            coords[0] += movement_vector[0];
            coords[1] += movement_vector[1];
            steps_taken += 1;

            match visited {
                Some(data) => {
                    if data.contains_key(&coords) {
                        println!("crossing at {:?}", coords);
                        let step_sum = steps_taken + data.get(&coords).unwrap();
                        if step_sum < least_steps {
                            println!("new least steps {:?}", step_sum);
                            least_steps = step_sum;
                        }

                        let distance = coords[0].abs() + coords[1].abs();
                        if distance < closest {
                            println!("new closest manhattan distance {:?}", distance);
                            closest = distance;
                        }
                    }
                },
                None => {
                    hs.insert(coords.clone(), steps_taken);
                }
            }
        }
    }
    hs
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents
        .trim()
        .split("\n").collect();
    let path1 = get_wire_path(lines.iter().nth(0).expect("missing line 1"), None);
    println!("wire 1 touched {} coordinates", path1.len());

    get_wire_path(lines.iter().nth(1).expect("missing line 2"), Some(&path1));

    Ok(())
}

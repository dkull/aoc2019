use num::integer::lcm;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(PartialEq, Clone, Debug)]
struct Planet {
    x: i32,
    y: i32,
    z: i32,
    vel_x: i32,
    vel_y: i32,
    vel_z: i32,
}

impl Planet {
    fn parse(line: &str) -> Planet {
        let re = Regex::new(r"[<>,=a-z]").unwrap();
        let replaced = re.replace_all(line, "");
        let result = replaced.split(" ").collect::<Vec<&str>>();
        Planet {
            x: result.get(0).unwrap().parse::<i32>().unwrap(),
            y: result.get(1).unwrap().parse::<i32>().unwrap(),
            z: result.get(2).unwrap().parse::<i32>().unwrap(),
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    fn kinetic_energy(&self) -> i32 {
        self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs()
    }
    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn load_code() -> Vec<Planet> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .trim()
        .split('\n')
        .map(|c| Planet::parse(c))
        .collect()
}

fn main() {
    let mut planets = load_code();
    let first_planets = planets.clone();
    let planet_count = first_planets.len();

    let mut step: u64 = 0;
    let mut cycle_lens = [0u64; 3];
    while cycle_lens[0] == 0 || cycle_lens[1] == 0 || cycle_lens[2] == 0 {
        for i in 0..planet_count {
            let p = &planets[i];
            let mut new_vel_x = p.vel_x;
            let mut new_vel_y = p.vel_y;
            let mut new_vel_z = p.vel_z;

            let (px, py, pz) = (p.x, p.y, p.z);
            for op in &planets {
                let (ox, oy, oz) = (op.x, op.y, op.z);
                if px > ox {
                    new_vel_x -= 1;
                } else if px < ox {
                    new_vel_x += 1;
                }

                if py > oy {
                    new_vel_y -= 1;
                } else if py < oy {
                    new_vel_y += 1;
                }

                if pz > oz {
                    new_vel_z -= 1;
                } else if pz < oz {
                    new_vel_z += 1;
                }
            }
            planets[i].vel_x = new_vel_x;
            planets[i].vel_y = new_vel_y;
            planets[i].vel_z = new_vel_z;
        }
        for i in 0..planet_count {
            planets[i].x += planets[i].vel_x;
            planets[i].y += planets[i].vel_y;
            planets[i].z += planets[i].vel_z;
        }

        step += 1;

        let mut step_total_energy = 0;
        for np in planets.iter() {
            step_total_energy += np.total_energy();
        }

        if step == 1000 {
            println!("p1 total energy {}", step_total_energy);
        }

        if cycle_lens[0] == 0
            && planets[0].x == first_planets[0].x
            && planets[0].vel_x == first_planets[0].vel_x
            && planets[1].x == first_planets[1].x
            && planets[1].vel_x == first_planets[1].vel_x
            && planets[2].x == first_planets[2].x
            && planets[2].vel_x == first_planets[2].vel_x
        {
            cycle_lens[0] = step;
        }
        if cycle_lens[1] == 0
            && planets[0].y == first_planets[0].y
            && planets[0].vel_y == first_planets[0].vel_y
            && planets[1].y == first_planets[1].y
            && planets[1].vel_y == first_planets[2].vel_y
            && planets[2].y == first_planets[2].y
            && planets[2].vel_y == first_planets[2].vel_y
        {
            cycle_lens[1] = step;
        }
        if cycle_lens[2] == 0
            && planets[0].z == first_planets[0].z
            && planets[0].vel_z == first_planets[0].vel_z
            && planets[1].z == first_planets[1].z
            && planets[1].vel_z == first_planets[1].vel_z
            && planets[2].z == first_planets[2].z
            && planets[2].vel_z == first_planets[2].vel_z
        {
            cycle_lens[2] = step;
        }
    }

    println!("p2 info {:?}", cycle_lens);
    println!(
        "p2: {}",
        lcm(cycle_lens[0], lcm(cycle_lens[1], cycle_lens[2]))
    );
}

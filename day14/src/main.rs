use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn pad(c: u32) -> String {
    let mut out = "".to_string();
    for _ in 0..c {
        out.push(' ');
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    batch_size: u32,
    batch_ore_cost: u32,
    inputs: Vec<(Chemical, u32)>,
}

impl Chemical {
    pub fn new(name: &str, batch_size: u32, recipe: Vec<(Chemical, u32)>) -> Chemical {
        let batch_ore_cost = if recipe[0].0.name == "ORE" {
            recipe[0].1
        } else {
            0
        };

        Chemical {
            name: name.into(),
            batch_size,
            inputs: recipe,
            batch_ore_cost,
        }
    }
    pub fn construct_ore() -> Chemical {
        Chemical {
            name: "ORE".into(),
            batch_size: 1,
            inputs: vec![],
            batch_ore_cost: 0,
        }
    }

    fn build_chain(
        &self,
        count: u32,
        mut surplus: HashMap<Chemical, u32>,
        mut build_counts: HashMap<Chemical, u32>,
        depth: u32,
    ) -> (u32, HashMap<Chemical, u32>, HashMap<Chemical, u32>) {
        let mut cost = 0;

        let mut built = 0;
        let surplus_count = surplus.entry(self.clone()).or_insert(0);
        /*println!(
            "{}> building {} {} (surplus {})",
            pad(depth),
            count,
            self.name,
            surplus_count
        );*/
        let mut surplus_use = 0;
        if self.batch_ore_cost > 0 {
            let surplus_count = surplus.entry(self.clone()).or_insert(0);
            if *surplus_count >= count {
                *surplus_count -= count;
                surplus_use += count;
            } else {
                let result = self.get_ore_cost_for(count - *surplus_count);
                surplus_use += *surplus_count;
                built += count - surplus_use + result.1;
                cost += result.0;
                *surplus_count = result.1;
                let build_count = build_counts.entry(self.clone()).or_insert(0);
                *build_count += count - surplus_use + result.1;
            }
        } else {
            if *surplus_count > 0 {
                built += *surplus_count;
                *surplus_count = 0;
            }
            while built < count {
                for (input_chem, input_count) in self.inputs.iter() {
                    let result =
                        input_chem.build_chain(*input_count, surplus, build_counts, depth + 1);
                    cost += result.0;
                    surplus = result.1;
                    build_counts = result.2;
                }
                built += self.batch_size;
            }
            let surplus_count = surplus.entry(self.clone()).or_insert(0);
            if built > count {
                *surplus_count = built - count;
            }
        }
        let surplus_count = surplus.entry(self.clone()).or_insert(0);
        /*println!(
            "{}> built {} {} with cost {} (used {} surplus -> {})",
            pad(depth),
            built,
            self.name,
            cost,
            surplus_use,
            surplus_count
        );*/

        (cost, surplus, build_counts)
    }

    pub fn get_ore_cost_for(&self, count: u32) -> (u32, u32) {
        let mut cost = 0;
        let mut produced = 0;

        while produced < count {
            produced += self.batch_size;
            cost += self.batch_ore_cost;
        }
        (cost, produced - count)
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();

    let mut chemicals: HashMap<String, Chemical> = HashMap::new();
    chemicals.insert("ORE".into(), Chemical::construct_ore());

    let mut parsed_lines = vec![];
    let mut i = 0;
    'nextline: while lines.len() > parsed_lines.len() {
        i %= lines.len();
        let line = lines[i];
        let thisline = i;
        i += 1;
        if parsed_lines.contains(&thisline) {
            continue;
        }

        //println!("parsing line {} {}", thisline, line);

        let mut recipe = vec![];
        let parts = line.split(" => ").collect::<Vec<&str>>();

        // parse input
        let inputs = parts[0].split(", ").collect::<Vec<&str>>();

        for input in inputs.iter() {
            let input_parts = input.split(' ').collect::<Vec<&str>>();
            let (input_count, input_name) =
                (input_parts[0].parse::<u32>().unwrap(), input_parts[1]);
            if chemicals.get(input_name).is_none() {
                //println!("skipping {} bc missing {}", line, input_name);
                continue 'nextline;
            }
            let input_chemical = chemicals.get(input_name).unwrap();
            recipe.push((input_chemical.clone(), input_count));
        }

        //parse output
        let output = parts[1].split(' ').collect::<Vec<&str>>();
        let (output_count, output_name) = (output[0].parse::<u32>().unwrap(), output[1]);

        let chemical = Chemical::new(&output_name, output_count, recipe);
        parsed_lines.push(thisline);
        chemicals.insert(chemical.name.clone(), chemical.clone());
    }
    for (_, chem) in chemicals.iter() {
        if chem.name != "FUEL" {
            continue;
        }
        let surplus = HashMap::new();
        let build_counts = HashMap::new();
        //println!("===");
        let (cost, surplus, build_counts) = chem.build_chain(1, surplus, build_counts, 0);
        println!("RESULT p1: {} cost {}", chem.name, cost);

        let surplus = HashMap::new();
        let build_counts = HashMap::new();
        let (cost, surplus, build_counts) = chem.build_chain(10, surplus, build_counts, 0);
        println!("RESULT p2: {} cost {}", chem.name, cost);

        let surplus = HashMap::new();
        let build_counts = HashMap::new();
        let (cost, surplus, build_counts) = chem.build_chain(100, surplus, build_counts, 0);
        println!("RESULT p2: {} cost {}", chem.name, cost);

        let surplus = HashMap::new();
        let build_counts = HashMap::new();
        let (cost, surplus, build_counts) = chem.build_chain(1000, surplus, build_counts, 0);
        println!("RESULT p2: {} cost {}", chem.name, cost);

        let surplus = HashMap::new();
        let build_counts = HashMap::new();
        let (cost, surplus, build_counts) = chem.build_chain(10000, surplus, build_counts, 0);
        println!("RESULT p2: {} cost {}", chem.name, cost);

        /*for bs in build_counts.iter() {
            println!("RESULT: build counts: {} {}", bs.0.name, bs.1);
        }*/
        /*for s in surplus.iter() {
            println!("surplus: {} {}", s.1, s.0.name);
        }*/
    }
}

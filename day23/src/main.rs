mod intcode;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use intcode::{load_code, spawn_processes};

fn main() {
    let code = load_code(2);

    let proc_count = 50;
    // data is sent in three packets
    let min_outputs = 3;
    let mut processes = spawn_processes(proc_count, code, vec![].into(), false);

    let mut queue: HashMap<usize, Vec<isize>> = HashMap::new();
    for i in 0..proc_count {
        queue.insert(i as usize, vec![i]);
    }
    queue.insert(255, vec![]);

    let mut last_sent_to_zero = 0;
    loop {
        let mut sent = false;
        for (net_id, proc) in (&mut processes).iter_mut().enumerate() {
            println!("> {} [{}]", net_id, sent);
            for inp in queue.get(&net_id).unwrap().iter() {
                sent = true;
                println!("pushing {} to machine {}", *inp, net_id);
                proc.input.push_back(*inp);
            }
            queue.get_mut(&net_id).unwrap().clear();
            proc.input.push_back(-1);

            proc.run_to_interrupt(min_outputs);
            if proc.output.len() >= 3 {
                println!("{} sending data to {:?}", net_id, proc.output.clone());
                let receiver = proc.output.pop_front().unwrap() as usize;
                let x = proc.output.pop_front().unwrap();
                let y = proc.output.pop_front().unwrap();
                if receiver == 255 {
                    queue.get_mut(&receiver).unwrap().clear();
                }
                queue.get_mut(&receiver).unwrap().push(x);
                queue.get_mut(&receiver).unwrap().push(y);
            }
        }
        if !sent {
            println!("NAT sending");
            let y = queue.get_mut(&255).unwrap().pop().unwrap();
            let x = queue.get_mut(&255).unwrap().pop().unwrap();
            queue.get_mut(&0).unwrap().push(x);
            queue.get_mut(&0).unwrap().push(y);
            if y == last_sent_to_zero {
                panic!("> p2: {}", y);
            }
            last_sent_to_zero = y;
        }
    }
}

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let bi = BufReader::new(io::stdin());
    let mut bo = BufWriter::new(io::stdout());

    let mut t_map = HashMap::new();
    let mut t_counter = 0;
    let mut t_all_counter = 0;
    let mut t_all_val = 0;

    let mut p_head = false;
    let mut p_counter = 0;
    for line in bi.lines() {
        let line = line.unwrap();
        if !p_head {
            p_counter = line.trim().parse().unwrap();
            p_head = true;
        } else {
            p_counter -= 1;
            let p_desc = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            match p_desc[0] {
                1 => {
                    t_map.insert(p_desc[1], (p_desc[2], t_counter));
                    t_counter += 1;
                } // set k = v
                2 => {
                    if let Some((v, c)) = t_map.get(&p_desc[1]) {
                        if *c < t_all_counter {
                            bo.write_all(format!("{t_all_val}\n").as_bytes()).unwrap();
                        } else {
                            bo.write_all(format!("{v}\n").as_bytes()).unwrap();
                        }
                    } else {
                        bo.write_all("-1\n".to_string().as_bytes()).unwrap();
                    }
                } // get k
                3 => {
                    t_all_val = p_desc[1];
                    t_all_counter = t_counter;
                    t_counter += 1;
                } // set_all k = v
                _ => panic!("wrong record"),
            }

            if p_counter == 0 {
                p_head = false;
                t_map.clear();
                t_counter = 0;
                t_all_counter = 0;
                t_all_val = 0;
            }
        }
    }

    bo.flush().unwrap();
}

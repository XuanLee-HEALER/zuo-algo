use std::collections::HashSet;

use sha2::{Digest, Sha256, digest::FixedOutputReset};

fn main() {
    let chars = ['a', 'b'];
    let mut all_str = Vec::with_capacity(1 << 20);
    let mut cont = String::with_capacity(20);
    gen_rand_str(0, 20, &chars, &mut cont, &mut all_str);
    println!("the amount of strings is {}", all_str.len());
    let mut hasher = Sha256::new();
    let mut set = HashSet::new();
    let mut ct = [0_usize; 10];
    all_str
        .iter()
        .map(|s| -> u128 {
            hasher.update(s.as_bytes());
            let b = (&hasher.finalize_fixed_reset()[..]).as_ptr() as *const [u8; 16];
            unsafe { u128::from_ne_bytes(*b) }
        })
        .for_each(|v| {
            set.insert(v);
            ct[(v % 10) as usize] += 1;
        });
    assert_eq!(set.len(), all_str.len());
    for i in 0..10 {
        println!("bucket [{}]: {}", i, ct[i])
    }
}

fn gen_rand_str(i: usize, n: usize, choose: &[char], cont: &mut String, res: &mut Vec<String>) {
    if i == n {
        res.push(cont.clone());
    } else {
        for &b in choose {
            cont.push(b);
            gen_rand_str(i + 1, n, choose, cont, res);
            cont.pop();
        }
    }
}

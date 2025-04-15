use rand::{Rng, rngs::ThreadRng, seq::SliceRandom, thread_rng};

fn main() {
    let tt = 100_000;
    let mut lives = 0;
    for _ in 0..tt {
        let gaol = Gaol::new(100, 50);
        if gaol.try_pick() {
            lives += 1;
        }
    }
    println!("try-pick live probability: {:.6}", lives as f64 / tt as f64);
    println!("theoretical probability: {:.6}", Gaol::calc(100, 50))
    // let max = 41;
    // let bag_l = 10;
    // let test_times = 100_000;
    // let mut ct = vec![0; max + 1];
    // for _ in 0..test_times {
    //     let mut pool = Pool::new(bag_l);
    //     for _ in 0..max {
    //         pool.add();
    //     }
    //     for &b in pool.bag() {
    //         ct[b] += 1;
    //     }
    // }
    // for i in 1..=max {
    //     println!("the amount of ball {} is {}", i, ct[i]);
    // }
}

struct Pool {
    bag: Vec<usize>,
    i: usize,
    rng: ThreadRng,
}

impl Pool {
    fn new(n: usize) -> Self {
        Self {
            bag: Vec::with_capacity(n),
            i: 1,
            rng: thread_rng(),
        }
    }

    fn add(&mut self) {
        let n = self.bag.capacity();
        if self.i <= n {
            self.bag.push(self.i);
        } else {
            if !self.disposal(self.i) {
                self.bag[self.rng.gen_range(0..n)] = self.i
            }
        }
        self.i += 1;
    }

    fn disposal(&mut self, i: usize) -> bool {
        !self.rng.gen_bool(self.bag.capacity() as f64 / i as f64)
    }

    fn bag(&self) -> &[usize] {
        &self.bag
    }
}

struct Gaol {
    p: usize,
    times: usize,
    boxes: Vec<usize>,
}

impl Gaol {
    fn new(n: usize, times: usize) -> Self {
        let mut rng = thread_rng();
        let mut ini: Vec<usize> = (0..n).collect();
        ini.shuffle(&mut rng);
        Self {
            p: n,
            times,
            boxes: ini,
        }
    }

    fn try_pick(&self) -> bool {
        let mut live = true;
        'out: for c in 0..self.p {
            let mut nxt = self.boxes[c];
            let mut ct = 1;
            while nxt != c {
                if ct > self.times {
                    live = false;
                    break 'out;
                }
                nxt = self.boxes[nxt];
                ct += 1;
            }
        }
        live
    }

    fn calc(n: usize, t: usize) -> f64 {
        let mut v = 0_f64;
        for c in t + 1..=n {
            v += 1.0 / c as f64;
        }
        1.0 - v
    }
}

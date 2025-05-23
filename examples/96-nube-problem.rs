fn main() {
    print_bi(101, 10);
    println!("{:b}", 101);
    assert_eq!(least_exp2(9), 3);
    assert_eq!(least_exp2(16), 4);
    assert_eq!(least_exp2(20_0000_0000), 30)
}

fn print_bi(mut x: i32, mut m: i32) {
    while x != 0 {
        let c = 1 << m;
        if x >= c {
            x -= c;
            print!("1")
        } else {
            print!("0")
        }
        m -= 1;
    }
    println!()
}

fn least_exp2(x: i32) -> i32 {
    let mut r = 0;
    while 1 << r <= x >> 1 {
        r += 1
    }
    r as i32
}

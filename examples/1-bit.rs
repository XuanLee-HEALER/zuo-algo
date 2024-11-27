fn print_i32_binary(num: i32) {
    let mut res = String::new();
    for i in (0..32).rev() {
        match num & (1 << i) {
            0 => res.push('0'),
            _ => res.push('1'),
        }
    }

    println!("{}", res)
}

fn main() {
    let i1 = 79;
    print!("79: ");
    print_i32_binary(i1);
    let i2 = -79;
    print!("-79: ");
    print_i32_binary(i2);

    assert_eq!(!i1 + 1, -79);
    assert_eq!(!i2 + 1, 79);
}

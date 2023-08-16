use std::io;

fn main() {
    let x: u8 = 9;
    let y: u8 = 10;

    /* type casting */
    /* type conversion */

    let x = 127i64;
    let q = 127_000 as i64;
    let w = 10_i32;

    let result = q / y as i64;
    println!("result = {}", result);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int_input: f64 = input.trim().parse().unwrap();
    println!("int_input = {}", int_input);
}

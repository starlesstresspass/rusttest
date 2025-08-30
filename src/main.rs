use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};

fn main() {
    println!("Hello, world!");
    let password_length: i32 = 15;
    let mut result = String::new();
    for i in 0..password_length{
        let number = thread_rng().gen_range(48..128);
        let ch: char = from_u32(number).unwrap();
        result.push(ch);
    }
    println!("{}", result);
}

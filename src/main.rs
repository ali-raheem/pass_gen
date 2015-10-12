extern crate rand;

use rand::{thread_rng, Rng};
use std::env;

fn main() {
    let len: usize = match env::args().nth(1) {
        Some(s) => { match s.parse() { Ok(s) => s, Err(_) => 8} },
	None => { 8 },
    };
    let pass: String = thread_rng().gen_ascii_chars().take(len).collect();
    println!("{}", pass);
}

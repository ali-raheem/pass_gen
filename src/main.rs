extern crate rand;

use rand::{thread_rng, Rng};
use std::env;

const DEFAULT_LEN: usize = 8;

fn main() {
    let len: usize = match env::args().nth(1) {
        Some(s) => { match s.parse() { Ok(s) => s, Err(_) => DEFAULT_LEN} },
	None => { DEFAULT_LEN },
    };
    let pass: String = thread_rng().gen_ascii_chars().take(len).collect();
    println!("{}", pass);
}

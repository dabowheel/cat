use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        let mut file = File::open(arg).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        print!("{}", s);
    }
}
#![allow(unused)]   // suppress compiler warnings related to unused variables, functions within its scope.

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name-osaurus?");
    let mut name = String::new();
    let greeting = "Nice to meet you, dude";
    io::stdin().read_line(&mut name)
        .expect("Didn't recieve input, dude !");

    println!("Hello {}osaurus! {}", name.trim_end(), greeting); //trim_end added to remove the new-line characters inserted by user
}


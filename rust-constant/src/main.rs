#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32= 1_000_000;
    const PI: f32 = 3.141592;
    // Unsigned integer : u64, usize
    // Signed integer : i64, isize
    let age = "27";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // Datatypes
    println!("Max limit of various Data-types :");
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max isize : {}", isize::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    let is_true = true;
    let my_grad = 'C';

    //Maths
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4: {}", num_3 + num_4);
    println!("5 - 4: {}", num_3 - num_4);
    println!("5 * 4: {}", num_3 * num_4);
    println!("5 / 4: {}", num_3 / num_4);
    println!("5 % 4: {}", num_3 % num_4);
    num_3 +=1;
    println!("num 3 updated : {}", num_3);

    // Random
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random from 1 to 100: {}", random_num);

}

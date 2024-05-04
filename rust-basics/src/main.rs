#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // if conditional
    let age: i32 = 21;
    if (age >= 1) && (age <= 18) {
        println!("You are precious little!");
    } else if (age ==21) || (age ==50) {
        println!("You are just entering new phase of your life !")
    } else if age >= 65 {
        println!("You are old but gold")
    } else {
        println!("Nobody cares about you, right ?")
    }

    //Ternary Operator
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote : {}", can_vote);

    //match conditional
    let age_2: i32 = 28;
    match age_2 {
        1..=18 => println!("You are precious little!"),
        21 | 50 => println!("You are just entering new phase of your life !"),
        65..=i32::MAX => println!("You are old but gold"),
        _ => println!("Nobody cares about you, right ?"),
    };

    //match conditional with comparison
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote ! Booohooo go home! "),
        Ordering::Greater => println!("Can Vote ! vote RIGHT NOW !"),
        Ordering::Equal => println!("You just gained the right to vote !")
    };

    //Array
    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("1st element in array: {}", arr_1[0]);
    println!("lenght of the array: {}", arr_1.len());

    //Loop
    // Printing Odd Numbers
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop{
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx+=1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}",arr_2[loop_idx]);
        loop_idx+=1;
    }

    //while
    let mut loop_idx_1 = 0;
    while loop_idx_1 < arr_2.len(){
        println!("Array element: {}", arr_2[loop_idx_1]);
        loop_idx_1+=1;
    }

    // for => requires creation of iterator of an array
    for val in arr_2.iter() {
        println!("for Val : {}", val)
    }

    // Tuples => fixed size of multiple different data types
    let my_tuple: (u8, String, f64) = (169, "Broncosaurus".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);

    let (age, name, mils) = my_tuple;
    println!("Age : {} billion years", age);

}

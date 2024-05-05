#![allow(unused)]


use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use rand::Rng;
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    //String
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    println!("{}", st3);
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        print!("{} ", char);
    }
    println!();

    let st4 : &str = "Random String";
    // convert the above to a heap allocated string
    let mut st5: String = st4.to_string();
    println!("{}",st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String Length : {}", st6.len());
    let mut loop_idx = 0;
    loop{
        println!("{}", byte_arr1[loop_idx]);
        loop_idx +=1;
        if loop_idx == byte_arr1.len() {
            break;
        }
    }
    println!("{}", byte_arr1[byte_arr1.len()-1]);
    st5.clear();
    println!("Clear String: {}", st5);

    let st6 = String::from("Just some");
    let st7 = String::from(" Words");
    let st8 = st6 + &st7; 
    // When & sign is used to denote the reference to the string
    // we can't add two Strings or &str but we can combine one String and &str instead
    println!("{}", st8);
    println!("{}", st7);
    // println!("{}", st6); => Can't use this as the st6 string is absorbed by the st8
    for char in st8.bytes(){
        println!("{}", char);
    }

    // Casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32:u32 = (int_u8 as u32) + (int2_u8 as u32);
    // Casting can be done on fly using as
    println!("Summation: {}", int3_u32);

    //Enums => create data-type with limited number of potential values
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    // We can implement function for enumerated types
    impl Day {
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        } 
    }

    let today = Day::Monday;

    // Tying Match here
    match today {
        Day::Monday => println!("GET BACK TO WORK !"),
        Day::Tuesday => println!("Not even close"),
        Day::Wednesday => println!("Uhhhghhh"),
        Day::Thursday => println!("Not close"),
        Day::Friday => println!("Closeee"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }
    println!("Is today the weekend {}", today.is_weekend())

    // Vectors -> Like array they can grow and only save value of same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("vec2 1st element :{}", vec2[0] );
    let second

    

}

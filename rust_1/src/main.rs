#![allow(unused)] // it's for not showing warning if some variable unused

// import lib using "use"
use rand::Rng; // not standard file
use std::cmp::Ordering;
use std::fs::File; // import fs or file system
use std::io; // basic input output
use std::io::{BufRead, BufReader, ErrorKind, Write}; // import different more lib in io lib

fn main() {
    println!("what is your name ? ");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    // taking input
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello, {} {}", name.trim_end(), greeting);

    const MILLION: u32 = 1_000_000;
    const PI: f32 = 3.14;
    let age: &str = "23";
    // two variable has same name but different datatype in rust we can use shadow
    let mut age: u32 = age.trim().parse().expect("age should be number ");
    age = age + 1;

    println!("I am {} of {}, needed this amount {} ", name, age, MILLION);
}

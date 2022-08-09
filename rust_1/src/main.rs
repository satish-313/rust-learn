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
    // generate random number

    let random_num = generate_random_number(15, 27);
    let age: String = random_num.to_string();
    // two variable has same name but different datatype in rust we can use shadow
    let mut age: u32 = age.trim().parse().expect("age should be number ");
    age = age + 1;

    // if and else condition in rust
    if_else_condition(age, name, MILLION);

    // match is simillar to swith in c++ or c
    match_condition(age);

    // compare
    match_cmp(age);
}

fn generate_random_number(s: i32, e: i32) -> i32 {
    return rand::thread_rng().gen_range(s..e);
}

fn if_else_condition(age: u32, name: String, million: u32) -> () {
    if (age >= 18) && (age < 20) {
        println!(
            "I am {} of {}, needed this amount {} ",
            name,
            age,
            million / 100
        );
    } else if (age >= 20) && (age <= 23) {
        println!(
            "I am {} of {}, needed this amount {} ",
            name,
            age,
            million / 50
        );
    } else if (age == 15) || (age == 25) {
        println!("I am {} of {}, needed this amount {} ", name, age, million);
    } else {
        println!("I am {} of {}, needed this amount {} ", name, age, 0);
    };
}

fn match_condition(age: u32) -> () {
    match age {
        15..=17 => println!("you can't vote in general election"),
        18 => println!("you are eligible for vote "),
        23 => println!("you are remove from casting vote"),
        18..=25 => println!("you are eligible to vote"),
        _ => println!("you are too old to vote"),
    };
}

fn match_cmp(age: u32) -> () {
    let elected_age = 25;
    match age.cmp(&elected_age) {
        Ordering::Less => println!("your age less than {}, not eligible", elected_age),
        Ordering::Equal => println!("your are eligible , your {} ", elected_age),
        Ordering::Greater => println!("try again you are eligible"),
    };
}

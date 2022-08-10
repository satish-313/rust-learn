#![allow(unused)]
use std::io;

pub fn print_name() -> String {
    println!("what is your name ? ");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";

    // taking input
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello, {} {}", name.trim_end(), greeting);
    return name;
}


pub fn print_age(name: &str) -> () {
    println!("Hi {} enter your age ",name);
    let mut ages = String::new();
    io::stdin().read_line( &mut ages).expect("Didn't receive number");

    let age: u32 = ages.trim().parse().expect("enter number");
    println!("Hi {} your age is : {}",name,age)
}   
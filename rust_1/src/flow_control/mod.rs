#![allow(unused)]

use std::cmp::Ordering;

pub fn if_else_condition(age: u32, name: &str, million: u32) -> () {
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

// match is simillar to swith in c++ or c
pub fn match_condition(age: u32) -> () {
    match age {
        15..=17 => println!("you can't vote in general election"),
        18 => println!("you are eligible for vote "),
        23 => println!("you are remove from casting vote"),
        18..=25 => println!("you are eligible to vote"),
        _ => println!("you are too old to vote"),
    };
}

pub fn match_cmp(age: u32) -> () {
    let elected_age = 25;
    match age.cmp(&elected_age) {
        Ordering::Less => println!("your age less than {}, not eligible", elected_age),
        Ordering::Equal => println!("your are eligible , your {} ", elected_age),
        Ordering::Greater => println!("try again you are eligible"),
    };
}

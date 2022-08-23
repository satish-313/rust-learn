#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn chapter2() -> () {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("guess the number");
        let mut guess:String = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("you guess less number"),
            Ordering::Greater => println!("you guess more number"),
            Ordering::Equal => { println!("you guessed won"); break;},
        }
    }
}

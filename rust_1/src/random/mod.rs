#![allow(unused)]

use rand::Rng;

pub fn generate_random_age() -> u32 {
    //generate random number

    let random_num = generate_random_number(15, 27);
    let age: String = random_num.to_string();
    //two variable has same name but different datatype in rust we can use shadow
    let mut age: u32 = age.trim().parse().expect("age should be number ");
    age = age + 1;

    return age;
}

fn generate_random_number(s: i32, e: i32) -> i32 {
    return rand::thread_rng().gen_range(s..e);
}

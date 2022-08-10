#![allow(unused)] // it's for not showing warning if some variable unused

// import lib using "use"
use rand::Rng; // not standard file
use std::cmp::Ordering;
use std::fs::File; // import fs or file system
use std::io; // basic input output
use std::io::{BufRead, BufReader, ErrorKind, Write}; // import different more lib in io lib

mod mod_test;
use crate::mod_test::mod_nest_test::nest_function;
mod input_output;
use crate::input_output::io_test::{print_name,print_age};
mod random;
use crate::random::generate_random_age;
mod flow_control;
use crate::flow_control::{if_else_condition,match_cmp,match_condition};
mod arr_loop;
use crate::arr_loop::{my_loop};

fn main() {
    // let name = print_name();
    // print_age(&name);

    // const MILLION: u32 = 1_000_000;
    // const PI: f32 = 3.14;

    // let age = generate_random_age();
    // if_else_condition(age, &name, MILLION);
    // match_condition(age);
    // match_cmp(age);

    my_loop()
}






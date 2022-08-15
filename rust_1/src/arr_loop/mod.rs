#![allow(unused)]

pub mod tuples;

pub fn my_loop() -> () {
    // array in rust
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("first is {}", arr_1[0]);
    println!("length of array is {}", arr_1.len());

    // loop through array
    let length = arr_1.len();
    let mut idx = 0;
    loop {
        if idx == length {
            break;
        };

        if arr_1[idx] % 2 == 0 {
            idx += 1;
            continue;
        };

        println!("val : {}", arr_1[idx]);
        idx += 1
    }
}

pub fn while_loop() -> () {
    let arr = [1,4,24,6,35,6];
    let mut idx = 0;
    
    while idx < arr.len() {
        if arr[idx] % 3 == 0 {
            println!("3 multiple are : {}",arr[idx]);
        };
        idx += 1;
    }
}

pub fn for_loop() -> () {
    let arr = [1,2,4,6,8,2,9,11];
    for num in arr.iter() {
        match num % 4{
            0 => { println!("four multiple : {}", num); },
            _ => { println!("not a four multiple") ;}
        }
    }
}
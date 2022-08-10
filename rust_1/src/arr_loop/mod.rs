#![allow(unused)]

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

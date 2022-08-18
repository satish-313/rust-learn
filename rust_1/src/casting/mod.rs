#![allow(unused)]

pub fn casting() -> () {
    let int_8: u8 = 5;
    let int2_8: u8 = 4;
    let int3_32:u32 = (int_8 as u32) + (int2_8 as u32);
    println!("{}",int3_32)
}

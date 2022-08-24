#![allow(unused)]

// every value in rust have some type to type to indicate value because rust is a static type
// language.


// rust have four primary type scalar types integer, floating point, boolean and char.

// integer has two type of value that is sign and unsing. sign are value that can be positive and
// negative, unsigned value can only be positive types. In interger different bit of number that is
// 8bit, 16bit, 32bit , 64bit, 128bit. more bit mean store larger value. value are is 2^(n-1) - 1.

// overflow in rust similar to other language if 8bit given a value greater then 255 then it throw
// error if wrap allow then it will over flow goes to 0 similar to %

// compound types are types is given to different value like array and tuples
// another type of compound types are array with one type of value

use std::io;

pub fn data_types() -> () {
   let x8:u8 = 10;
   let x16:u16 = 80;
   let x32:u32 = 2434;

   println!("8bit {x8}, 16bit {x16}, 32bit {x32}"); 
   println!("8bit max {} , 16bit max {} , 32bit max {}", u8::MAX, u16::MAX, u32::MAX);

   let f:f32 = 32.0;
   let f1: f64 = 64.0;

   let t: bool = true;
   let f: bool = false;

   let ch:char = 'a';

   let tup: (&str, i32, f64, char) = ("satish" , 24 , 89.40 , 'A');

   println!("Name : {}",tup.0);

   let (name,age,percentage,grade) = tup;

   println!("I am {} of {}, pass with {}% grade {}",name,age,percentage,grade);

   let mut arr:[u32;5] = [1,2,3,4,5];

   loop {
       println!("enter a index number 0 to 4"); 
       let mut index = String::new();
       io::stdin().read_line(&mut index).expect("enter a valid number");
       let index:usize = match index.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };

       if (index >= arr.len()) {
           continue;
       }
       else {
          println!("index value is {}",arr[index]);
           break;
       }
   }
}

#![allow(unused)]

pub fn tuple_learn() -> () {
  let my_tuple = ("satish".to_string(), 24, 90.4);
  println!("1st index of tuples : {}", my_tuple.0);
  let (name,age,percentage) = my_tuple;
  println!("my {} , {} and final year percentage {}",name,age,percentage);
  // println!("my tuples {} ",my_tuple.0); it will throw a error string heap memory not stack memory
}

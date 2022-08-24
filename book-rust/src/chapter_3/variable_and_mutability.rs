#![allow(unused)]

// in rust variable default is immutable becase of memory safety
// we can use  mut variable by mut word in rust

// constant in rust in a way to declare a viriable in rust but have some characteristic like
// constant all letter should be uppercase in space present underscore and type must be explitily
// and value most in present in complile time if in runtime calculate value most be thorw error

// shadowing in rust is important feature or bug if you come from anthor language in shadowing we
// can declare same variable use same time and name but previous will shadow by new till value 
// exist

pub fn variable_and_mutability() -> () {
    let x = 5; // x = 6 it will throw error
    let mut y = 6;
    y = 10; // this will not thorw error
    println!("x : {x}, y: {y}");

    const SUR_NAME:&str = "PRADHAN";
    println!("{SUR_NAME}");
    
    let sx = 20;
    let sx = 20 * 2;

    {
        let sx = sx + 2;
        println!("the value of x in inner scope {sx}");
    }
    
    println!("the value of x after the scope {sx}");
}

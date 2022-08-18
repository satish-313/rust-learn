#![allow(unused)]

pub fn string_in() -> () {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    // for word in st1.split_whitespace() {
    //     println!("{}",word);
    // }

    let mut st2 = st1.replace("A", "Another");
    println!("{}",st2);

    let mut st3 = String::from("a z x a c c d f c k q");
    let mut v1: Vec<char> = st3.chars().collect();

    v1.sort(); // sort
    v1.dedup(); // remove deplicate
    // for i in v1 {
    //     println!("{}",i)
    // }

    let st4 = "random string";
    let mut st5: String = st4.to_string();
    println!("{}",st5);
    let bytes_arr1 = st5.as_bytes();
    let st6 = &bytes_arr1[0..6];
    for i in st6 {
        println!("{}",i);
    };

    println!("length of string {}",st5.len());
    st5.clear() // empty the string if mut

}

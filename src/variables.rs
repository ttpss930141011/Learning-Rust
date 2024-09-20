use std::io::{self, Read};

pub(crate) fn variables() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // This will not compile because x is immutable
    // println!("The value of x is: {}", x);
    //
    // let y = 5;
    // let y = y + 1;
    // {
    //     let y = y * 2;
    //     println!("The value of y in the inner scope is: {}", y);
    // }
    // println!("The value of y is: {}", y);

    // for b in io::stdin().bytes() {
    //     let c = b.unwrap() as char;
    //     println!("{}", c);
    // }

    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{word}");
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
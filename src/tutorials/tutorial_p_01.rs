// use rand::{Rng, thread_rng};
use std::io::stdin;

pub fn play_it() {
    println!("What is your name?");
    let mut name: String = "".to_string();
    // let greeting = "Nice to meet  you";

    stdin().read_line(&mut name).expect("Didn't recieve input");

    // println!("Hello {}, {}", name.trim_end(), greeting);

    const ONE_BILL: u32 = 1_000_000_000;
    let age: &str = "20";
    let mut age: u8 = age.trim().parse().expect("age wasn't assigned a number");
    age = age + 1;
    println!(
        "I'm {}, {} years old and I want {}",
        name.trim(),
        age,
        ONE_BILL
    );
}

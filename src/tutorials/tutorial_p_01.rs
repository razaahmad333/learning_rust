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

pub fn conditionals() {
    let age: u8 = 72;

    if (age >= 1) && (age <= 18) {
        println!("not adult, but teenagers ")
    } else if (age == 21) || (age == 31) {
        println!("oddy")
    } else if age > 65 {
        println!("adult")
    }

    // rust have a ternary operator;
    let can_vote = if age > 18 { true } else { false };
    println!(
        "can vote : {}",
        if can_vote {
            if age % 2 == 0 {
                "even person"
            } else {
                "odd person"
            }
        } else {
            "no"
        }
    );
}

use std::cmp::Ordering;

pub fn play_it() {
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

    let money = 30002;

    match money {
        1..=100 => println!("poor man, I'm not being racist here 🤝"),
        1000 | 2000 | 3000 => println!("money in Ks"),
        3001..=i32::MAX => println!("rich"),
        _ => println!("Live"),
    }

    let p_h: u32 = 12;
    let neutral: u32 = 7;

    match p_h.cmp(&neutral) {
        Ordering::Less => println!("Acidic"),
        Ordering::Equal => println!("Neutral"),
        Ordering::Greater => println!("Basic"),
    }
}

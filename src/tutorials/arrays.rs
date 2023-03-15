pub fn play_it() {
    println!("Lets learn arrays and a tupple");

    let me = ("Ahmad Raza".to_string(), 21, "JMI".to_string());

    let mut marks: [i16; 5] = [23, 32, 43, 12, 23];
    marks[0] = 10;

    let mut sum: i16 = 0;
    let mut cnt: usize = 0;

    println!(
        "Hii my name is {}, \nI'm {}, \nand I am graduated from {}",
        me.0, me.1, me.2
    );

    let [m1, m2, m3, m4, m5] = marks;
    println!("{}, {}, {}, {}, {}", m1, m2, m3, m4, m5);

    loop {
        let mut mark = &marks[cnt];
        println!("mark from variable -> {}", mark);

        let temp = if *mark % 2 == 0 { *mark } else { *mark - 1 };
        mark = &temp;

        sum += *mark;
        println!("mark from updated array maybe -> {}", mark);
        cnt += 1;

        if cnt == 5 {
            break;
        } else {
            continue;
        }
    }

    // while cnt < marks.len() {
    //     println!(" mark {} ", marks[cnt]);
    //     sum += marks[cnt];
    //     cnt += 1;
    // }

    // for mark in marks {
    //     println!("mark -> {}", mark);
    //     sum += mark;
    //     cnt += 1;
    // }

    // for mark in marks.iter() {
    //     println!("mark -> {}", mark);
    //     sum += mark;
    //     cnt += 1;
    // }

    println!("students -> {}", marks.len());
    println!("avg marks -> {}", sum / (cnt as i16));
}

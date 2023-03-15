use std::ops::Add;

pub fn play_it() {
    println!("let learn functions");
    let x = 2;
    let y: i32 = 10;
    get_sum(&x, y);

    let (fname, lname) = get_name_split(&"Ahmad Raza".to_string());
    println!("{}, {}", fname, lname);

    let list = vec![1, 2, 3, 4, 5, 6];
    println!("{}", get_sum_list(&list));

    println!("4 + 5 = {}", get_sum_gen(4, 5));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn get_sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for val in list.iter() {
        sum = sum + *val;
    }

    return sum;
}

fn get_sum(x: &i32, y: i32) {
    println!("{x} + {y} = {}", x + y);
}

fn get_name_split(name: &String) -> (String, String) {
    let split: Vec<&str> = name.split(" ").collect::<Vec<&str>>();
    return (split[0].to_string(), split[1].to_string());
}

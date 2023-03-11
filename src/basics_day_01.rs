pub fn basics_day01() {    
    println!("Hello, world!");

    const NUMBER: i32 = 12;
    println!("{}", NUMBER);

    const FLOATING_NUMBER: f32 = 12.21;
    println!("{}", FLOATING_NUMBER);

    let big_number:i64 = 122222222222222;   
    let number = 1_000_000;
    println!("{}", number);

    let _small_negative_integer : i8= -12;
    println!("{}", _small_negative_integer);

    let big_number_2 = big_number;
    println!("{}", big_number_2);
    println!("{}", big_number);

    // let some:f32 = 111.0;

    let smile:char = '😀';
    println!("{}", smile);
    let is_good = true;

    println!("{}", is_good);

}

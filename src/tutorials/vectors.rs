pub fn play_it() {
    let mut incomes: Vec<i16> = Vec::new();
    incomes.push(200);
    incomes.push(230);
    incomes.push(150);

    println!("{:?}.len() =  {}", incomes, incomes.len());
    println!("{}", incomes[2]);
    // let third: &i16 = &incomes[3];
    match incomes.get(3) {
        Some(third) => println!("Element exist=> {}", third),
        None => println!("Element do not exists"),
    }
}

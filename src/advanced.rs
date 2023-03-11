#[derive(Debug)]

pub(crate) enum Status {
    ACTIVE,
    INACTIVE
}

pub(crate) fn main_advanced_awesome() {
    println!("lets learn some new concept");


    let active = Status::ACTIVE;
    let mangoes = 12;
    let mut apples = 12;
    let sum = apples + mangoes;
    apples = 20;
    println!("Number of apples: {} {}", apples, sum);

    
    println!("{:?}", active)
}
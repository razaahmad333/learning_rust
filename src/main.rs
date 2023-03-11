#![allow(unused)]

mod tutorials {
    pub mod arrays;
    pub mod conditionals;
    pub mod tutorial_p_01;
}

// use tutorials::tutorial_p_01::{conditionals, play_it as _play_it_1};
// use tutorials::conditionals;
// use tutorials::tutorial_p_01;
use tutorials::arrays;
// mod advanced;
// use advanced::{main_advanced_awesome, Status};

fn main() {
    // main_advanced_awesome();
    // let inactive: advanced::Status = advanced::Status::INACTIVE;
    // let inactive: Status = Status::INACTIVE;
    // println!("{:?}", inactive);
    // println!("{/}", isize::MAX)
    // _play_it_1()
    // conditionals::play_it();
    arrays::play_it();
    // tutorial_p_01()
}

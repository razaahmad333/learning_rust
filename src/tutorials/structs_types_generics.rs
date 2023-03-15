pub trait Human {
    fn new(name: String, age: i8, money: i32) -> Self;
    fn get_future_age(&self, after: i8) -> i8;
    fn print_current_balance(&self);
}

struct Man {
    name: String,
    age: i8,
    money: i32,
}

impl Man {
    pub fn say_hii(&self) {
        println!("hii, my name is {}", self.name);
    }
}
struct Woman {
    name: String,
    age: i8,
    money: i32,
}

impl Human for Man {
    fn new(name: String, age: i8, money: i32) -> Man {
        return Man { name, age, money };
    }
    fn get_future_age(&self, after: i8) -> i8 {
        return self.age + after;
    }

    fn print_current_balance(&self) {
        println!(
            "The current balance of {} is ${}",
            self.name,
            self.money * (self.age as i32)
        );
    }
}

impl Human for Woman {
    fn new(name: String, age: i8, money: i32) -> Woman {
        Woman { name, money, age }
    }
    fn get_future_age(&self, after: i8) -> i8 {
        self.age + after
    }

    fn print_current_balance(&self) {
        println!(
            "The current balance of {} is ${}",
            self.name,
            self.money * ((self.age + 12) as i32)
        );
    }
}

struct Customer {
    name: String,
    address: String,
    balance: f32,
}

pub fn play_it() {
    let mut ahmad: Customer = Customer {
        name: "Ahmad Raza".to_string(),
        address: String::from("SOme where"),
        balance: 32.1,
    };

    ahmad.balance -= 12.1;

    println!("{}", ahmad.name);
    println!("{}", ahmad.address);
    println!("{}", ahmad.balance);

    let sam: Man = Man::new(String::from("Sam Wilson"), 20, 1002);
    let lisa: Woman = Human::new(String::from("Lisa Rosem"), 22, 200);
    println!("{}", sam.get_future_age(5));
    sam.say_hii();
    sam.print_current_balance();
    lisa.print_current_balance();

    print_details(&lisa);

    say_something(None);
}

fn say_something(num: Option<i8>) {
    println!("{}", num.unwrap_or(2));
}

pub fn print_details<T: Human>(human: &T) {
    println!("{}", &human.name);
}

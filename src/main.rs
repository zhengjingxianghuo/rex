use::rex::CB;

struct ME {
    name: String,
    age: u32,
}

impl CB for ME {
    fn NS(&self) -> i32 {
        30
    }
}

fn main() {
    println!("Hello, world!");
}

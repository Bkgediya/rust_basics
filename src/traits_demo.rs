

struct User {
    name :String,
    age : u32
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}


pub trait Summary {
    fn summarize(&self) -> String;
}



pub fn run() {
    let user = User {
        name : String::from("John"),
        age: 100,
    };

    println!("{}", user.summarize());
    println!("Hello from trait");
}
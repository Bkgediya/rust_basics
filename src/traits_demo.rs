

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

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn run() {
    let user = User {
        name : String::from("John"),
        age: 100,
    };

    println!("{}", user.summarize());

    notify(&user);
    println!("Hello from trait");
}
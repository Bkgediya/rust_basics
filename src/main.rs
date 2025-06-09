use std::{collections::HashMap, fs::read_to_string};
use chrono::{ Utc };
mod iterators;
mod generics;
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),   
}

struct User {
    first_name : String,
    last_name : String,
    is_active: bool,
    age: i32
}

impl User {
    fn get_full_name(&self) -> String {
        return format!("{} {}",self.first_name,self.last_name);
    }
}

fn vector_with_event(vec : &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for i in vec {
        if *i % 2 == 0 {
            new_vec.push(*i);
        }
    }

    return new_vec;
}

fn main() {
    generics::run();
    iterators::run();

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(5);
    vec.push(6);

    let new_vec = vector_with_event(&vec);
    println!("{:?}",new_vec);


    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("harsh"), 50);

    let value = hashmap.get("harsh");

    match value {
        Some(value) => println!("{}",value),
        None => println!("Not found"),
    }

    let now = Utc::now();
    println!("{}",now);

    println!("{}",is_event(7));
    println!("{}",fib(8));
    println!("{}",get_str_length(String::from("Hello world!")));

    let user1 = User {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        is_active: true,
        age: 100
    };  

    println!("{},{} ,{} ,{}",user1.age,user1.first_name,user1.last_name,user1.is_active);
    println!("{}",user1.get_full_name());

    let circle = Shape::Circle(10.0);
    calculate_area(circle);

    let square = Shape::Square(10.0);
    calculate_area(square);

    let rectangle = Shape::Rectangle(10.0,10.0);
    // let direction: Directions = Directions::NORTH;
    calculate_area(rectangle);

    let index = find_first_a(String::from("Hello world!"));
    match index {
        Some(value) => println!("Found a at index {}",value),
        None => println!("Not found"),
    }

    let result = read_to_string("a.txt");
    match result {
        Ok(result) => println!("{}",result),
        Err(error) => println!("{}",error),
    }
}

fn calculate_area(shape:Shape) -> f64 {
    let area  =  match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width,height) => width * height,
    };
    return area; 
}

fn find_first_a(s :String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32); 
        }
    }

    return None;
}

fn is_event(num : i32) -> bool {
    return num % 2 == 0;
}

fn fib(num :u32) -> u32{
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return  1;
    }

    return fib(num - 1) + fib(num - 2);
}

fn get_str_length(str: String) -> usize {
    str.chars().count()
}
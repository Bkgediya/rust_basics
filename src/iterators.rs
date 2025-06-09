use std::vec;



pub fn run() {
    // iter  -> immutable reference
    // iter_mut -> mutable reference
    // iter_iter  -> takes ownership

    let numbers = vec![1,2,3,4,5];
    // Gives &T
    // Doesn't allow modifying the elements
    // Does not consume the collection
    for num in numbers.iter() {
        println!("{}", num);
    }
    println!("{:?}",numbers);

    // Gives &mut T
    // Allows in-place mutation
    // Does not consume the collection
    // The collection must be declared mut
    let mut numbers2 = vec![1,2,3,4,5];

    for num in numbers2.iter_mut() {
        *num *= 2
    }

    println!("{:?}",numbers2);

    println!("Hello from iterators");

    // slices and string
    let word = String::from("hello world what are you doing!");

    let sliceEx = &word[0..5];
    println!("{}",sliceEx);

    let literal = "done and clear";
    println!("{}",literal)
}
/*
 * - All value in Rust have a owner.
 * - Each one value can have only one owner at a time.
 * - The lifetime of a value is the scope for which the value is valid.
 * - When the owner goes out of scope, the value will be dropped.
 */

pub fn example_basic() {
    {
        let s = String::from("hello");
        println!("{}", s);
    }
    // println!("{}", s); // Error: value borrowed here after move
}

pub fn example_lifetime() {
    let s1: &String;
    {
        let s2 = String::from("hello");
        s1 = &s2;
        println!("{}", s1);
    }
    // println!("{}", s1); // Error: `s2` does not live long enough
}

pub fn example_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // Error: value borrowed here after move
    println!("{}", s2);
}

pub fn example_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn example_copy() {
    /*
     * Fixed values ​​are stored on the stack and are copied
     * The data type must implement the 'Copy' trait
     * Some implementations of the 'Copy' trait: all integers, all floating point numbers, booleans, characters
     * - All integers
     * - All floating point numbers
     * - Booleans
     * - Characters
     * - Tuples (if all elements are 'Copy')
     */
    let x = 5.5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let array = [1, 2, 3, 4, 5];
    let array_copy = array;
    println!("{:?}", array);
    println!("{:?}", array_copy);
}

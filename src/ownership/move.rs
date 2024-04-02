pub fn example_a() {
    let name = String::from("Sirius Beck");
    show_name(name);
    // println!("Name: {}", name);

    let age = 27;
    show_age(age);
    println!("Age: {}", age);
}

/// show_name() takes ownership of the variable 'name'
/// 'name' is no longer valid after this function call because it was moved to the function
fn show_name(name: String) {
    println!("Name: {}", name)
}

/// show_age() takes a copy of the variable 'age' (don't take ownership)
/// 'age' is still valid after this function call
fn show_age(age: i32) {
    println!("Age: {}", age)
}

pub fn example_b() {
    let name = get_new_name();
    println!("{}", name);

    let (name, s) = calculate_size(name);
    println!("{} has {} letters", name, s);
}

/// get_new_name() returns a new String
/// The ownership of the variable 'name' is moved to the caller
fn get_new_name() -> String {
    let name = String::from("Maria");
    name
}

/// calculate_size() takes ownership of the variable 'name'
/// The ownership of the 'name' and 'size' is moved to the caller in a tuple
fn calculate_size(name: String) -> (String, usize) {
    let size = name.len();
    (name, size)
}

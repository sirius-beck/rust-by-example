pub fn immutable() {
    let x = 10;
    // x = 10; /* cannot mutate immutable variable `x` */
    println!("x = {}", x)
}

pub fn mutables() {
    let mut x = 10;
    let y = x;

    let show_result = |x: i32, y: i32| println!("x, y = {}, {}", x, y);

    show_result(x, y);
    x = 5;
    show_result(x, y);
}

pub fn constants() {
    const Z: i32 = 50;
    println!("The value of 'Z' is: {}", Z);
}

pub fn shadowing() {
    let a = 50;
    println!("The value of 'a' is: {}", a);

    let a = "Sirius Beck";
    println!("The value of 'a' is: {}", a);

    let a = [1, 2, 3, 4, 5];
    println!("The value of 'a' is: {:?}", a);
}

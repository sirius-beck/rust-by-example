pub fn arithmetic() {
    let x = 10;
    let y = 5;

    println!("x = {}", x);
    println!("y = {}\n", y);

    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x % y = {}", x % y);
}

pub fn relational() {
    let x = 10;
    let y = 5;

    println!("x = {}", x);
    println!("y = {}\n", y);

    println!("x == y = {}", x == y);
    println!("x != y = {}", x != y);
    println!("x > y = {}", x > y);
    println!("x < y = {}", x < y);
    println!("x >= y = {}", x >= y);
    println!("x <= y = {}", x <= y);
}

pub fn logical() {
    let x = true;
    let y = false;

    println!("x = {}", x);
    println!("y = {}\n", y);

    println!("x && y = {}", x && y);
    println!("x || y = {}", x || y);
    println!("!x = {}", !x);
    println!("!y = {}", !y);
}

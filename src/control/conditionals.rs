pub fn example() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);

    let z = match x {
        5 => 10,
        6 => 15,
        _ => 20,
    };
    println!("z is {}", z);
}

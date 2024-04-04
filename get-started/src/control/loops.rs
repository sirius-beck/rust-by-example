pub fn example_for_range() {
    println!("for a in 0..5 {{...}}");
    for a in 1..5 {
        println!("  a = {}", a);
    }

    println!("\nfor b in 1..=5 {{...}}");
    for b in 1..=5 {
        println!("  b = {}", b);
    }

    println!("\nfor c in (1..5).rev() {{...}}");
    for c in (1..5).rev() {
        println!("  c = {}", c);
    }

    println!("\nfor d in (1..=10).step_by(2) {{...}}");
    for d in (1..=10).step_by(2) {
        println!("  d = {}", d);
    }
}

pub fn example_for_array() {
    let array = [1, 2, 3, 4, 5];

    println!("for i in 0..array.len() {{...}}");
    (0..array.len()).for_each(|i| {
        println!("  array[{}] = {}", i, array[i]);
    });

    println!("\nfor value in array {{...}}");
    for value in array {
        println!("  value is {}", value);
    }

    println!("\nfor (i, value) in array.iter().enumerate() {{...}}");
    for (i, value) in array.iter().enumerate() {
        println!("  array[{}] = {}", i, value);
    }
}

pub fn example_while() {
    let mut a = 0;
    println!("while a < 5 {{...}}");
    while a < 5 {
        println!("  a = {}", a);
        a += 1;
    }
}

pub fn example_loop() {
    let mut b = 0;
    println!("\nwhile b < 5 {{...}}");
    loop {
        println!("  b = {}", b);
        b += 1;
        if b >= 5 {
            break;
        }
    }
}

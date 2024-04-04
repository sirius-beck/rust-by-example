pub fn example() {
    basic_function();
    function_with_parameters(1, 2);

    let result = function_with_return();
    println!("Function with return: {}", result);

    let result = function_with_return_and_parameters(5, 6);
    println!("Function with return and parameters: 5 + 6 = {}", result);
}

fn basic_function() {
    println!("Basic function");
}

fn function_with_parameters(a: i32, b: i32) {
    println!("Function with parameters: a = {}, b = {}", a, b);
}

fn function_with_return() -> i32 {
    // The last expression in the function is the return value
    // This is equivalent to: return 27;
    27
}

fn function_with_return_and_parameters(a: i32, b: i32) -> i32 {
    a + b
}

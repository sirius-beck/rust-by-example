pub fn example_map() {
    let power_of_two: Vec<i32> = (1..=10).map(|x: i32| x.pow(2)).collect();
    println!("{:?}", power_of_two);
}

pub fn example_filter() {
    let is_prime_number = |n: i32| -> bool {
        if n < 2 {
            return false;
        }

        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }

        return true;
    };

    let numbers: Vec<i32> = (1..=10000)
        .filter(|&x| is_prime_number(x))
        .filter(|&x| x.to_string().ends_with('7'))
        .collect();
    println!("{:?}", numbers);
}

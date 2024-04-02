pub fn example() {
    // fixed and immutable size
    let tuple: (i32, i32, i32) = (1, 2, 3);
    println!("tuple = {:?}", tuple);
    println!("tuple.0 = {}", tuple.0);

    let (a, b, c) = tuple;
    println!("Tuple elements = {} {} {}", a, b, c);

    // fixed and changeable size
    let mut array: [i32; 10] = [0; 10];
    array[0] = 1;
    array[4] = 5;
    array[9] = 10;
    println!("Array: {:?}", array);
    println!("array[0]: {}", array[0]);

    // dinamically sized type (DST)
    // parts of an array
    let mut slice: &[i32] = &array[1..4];
    println!("Slice: {:?}", slice);

    slice = &array[0..5];
    print!("slice.iter().for_each() = ");
    slice.iter().for_each(|x| print!("{} ", x));
    println!();

    // dynamic structure
    // variable and changeable size
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    println!("Vector = {:?}", vector);

    let mut vector = vec![7, 8, 9];
    vector.push(10);
    println!("Vector = {:?}", vector);
    println!("vector[0] = {}", vector[0]);
    println!("vector.pop() = {}", vector.pop().unwrap());
    println!("Vector = {:?}", vector);
}

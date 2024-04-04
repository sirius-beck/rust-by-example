pub fn example() {
    let active: bool = true;
    println!("Boolean: {}", active);

    let characters: char = 'a';
    println!("Char: {}", characters);

    // &str (inferred), str
    let name: &str = "Sirius";
    println!("String: {}", name);

    let mut name: String = String::from("Sirius");
    name.push_str(" Beck");
    println!("String: {}", name);

    // i8, i6, i32 (inferred), i64, i28, isize (defined by system architecture)
    // u8, u6, u32, u64, u28, usize (defined by system architecture)
    let amount: i32 = 10;
    println!("Inteiro: {}", amount);

    // f32, f64 (inferred)
    let price: f64 = 10.99;
    println!("Ponto flutuante: {}", price);
}

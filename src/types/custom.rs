use std::fmt::{Display, Formatter, Result};

pub fn example_structs() {
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "Siriu".to_string(),
        age: 27,
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}

pub fn example_enums() {
    #[allow(dead_code)]
    enum Color {
        Purple,
        Red,
        Green,
        Blue,
    }

    let color = Color::Purple;

    println!("Color: {}", color);

    match color {
        Color::Purple => println!("Purple"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match self {
                Color::Purple => write!(f, "Purple"),
                Color::Red => write!(f, "Red"),
                Color::Green => write!(f, "Green"),
                Color::Blue => write!(f, "Blue"),
            }
        }
    }

    println!("Color: {}", Color::Purple);
}

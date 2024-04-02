/*
 * - You can have multiple immutable references to a resource, but only one mutable reference
 * - Immutable references cannot be converted to mutable references and vice versa
 */

pub fn example_immutable_ref() {
    let calculate_words_count = |text: &str| -> usize {
        // text.push_str("..."); /* error[E0596]: cannot borrow `text` as mutable, as it is not declared as mutable */
        text.split_whitespace().count()
    };

    let text = "Hello, World!";
    let words_count = calculate_words_count(text);
    println!("Text: {}", text);
    println!("Words count: {}", words_count);
}

pub fn example_mutable_ref_a() {
    let add_surname = |name: &mut String| {
        name.push_str(" Beck");
    };

    let mut name = String::from("Sirius");
    println!("Name: {}", name);

    add_surname(&mut name);
    println!("Name: {}", name);
}

pub fn example_mutable_ref_b() {
    let add_surname = |name: &mut String| {
        name.push_str(" Beck");
    };

    let mut name = String::from("Sirius");
    println!("Name: {}", name);

    let name_ref_1 = &mut name;
    // let name_ref_2 = &mut name; /* error[E0499]: cannot borrow `name` as mutable more than once at a time */
    add_surname(name_ref_1);
    // add_surname(&mut name); /* error[E0499]: cannot borrow `name` as mutable more than once at a time */
    println!("Name: {}", name_ref_1);

    let name_ref_2 = &mut name; /* Now it's possible, because the previous reference is not being used */
    add_surname(name_ref_2);
    println!("Name: {}", name_ref_2);

    add_surname(&mut name); /* Now it's possible, because the previous references are not being used */
    println!("Name: {}", name);
}

pub fn example_pending_ref() {
    // let points_to_nothing = generate_pending();
    // println!("Pending reference {}", points_to_nothing);

    let no_pending = no_pending();
    println!("No pending {}", no_pending);
}

// fn generate_pending() -> &String {
//     let text = String::from("Pending reference");
//     &text // returns a reference to a local variable that will be dropped
// }

fn no_pending() -> String {
    let text = String::from("No pending");
    text // returns the local variable itself
}

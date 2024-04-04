/*
 * - A slice is a reference to a part of a string or array
 * - It doesn't have ownership
 */

#[allow(unused_assignments)]
#[allow(unused_mut)]
pub fn example() {
    let mut text = String::from("Sirius Beck is a Rust developer");
    let letter = 'B';
    let word = first_word_with(letter, &text);

    println!("Text: {}", text);
    println!("First word with {}: {}", letter, word);

    // text = String::from("Sirius is a Rust developer"); /* cannot assign to `text` because it is borrowed */
}

fn first_word_with(letter: char, text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == letter as u8 {
            return text[i..].split_whitespace().next().unwrap();
        }
    }

    ""
}

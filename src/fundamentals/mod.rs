mod first;
mod operators;
mod variables;

use crate::utils::terminal::{clear_screen, show_menu, wait_enter};

pub fn execute() {
    loop {
        let items = [
            "First Example",
            "Variables -Immutable",
            "Variables - Mutables",
            "Variables - Constants",
            "Variables - Shadowing",
            "Operators - Arithmetic",
            "Operators - Relational",
            "Operators - Logical",
        ];

        let selected = show_menu("Fundamentals", &items, false);

        clear_screen();

        match selected {
            1 => first::example(),
            2 => variables::immutable(),
            3 => variables::mutables(),
            4 => variables::constants(),
            5 => variables::shadowing(),
            6 => operators::arithmetic(),
            7 => operators::relational(),
            8 => operators::logical(),
            _ => break,
        }

        println!();
        wait_enter();
    }
}

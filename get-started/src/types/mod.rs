mod basics;
mod custom;
mod extended;
mod sequences;

use crate::utils::terminal::{clear_screen, show_menu, wait_enter};

pub fn execute() {
    loop {
        let items = [
            "Basics",
            "Sequences",
            "Extended",
            "Custom - Structs",
            "Custom - Enums",
        ];

        let selected = show_menu("Types", &items, false);

        clear_screen();

        match selected {
            1 => basics::example(),
            2 => sequences::example(),
            3 => extended::example(),
            4 => custom::example_structs(),
            5 => custom::example_enums(),
            _ => break,
        }

        println!();
        wait_enter();
    }
}

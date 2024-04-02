mod fns;
mod lambda;

use crate::utils::terminal::{clear_screen, show_menu, wait_enter};

pub fn execute() {
    loop {
        let items = ["Basics", "Map", "Filter"];
        let selected = show_menu("Functions", &items, false);

        clear_screen();

        match selected {
            1 => fns::example(),
            2 => lambda::example_map(),
            3 => lambda::example_filter(),
            _ => break,
        }

        println!();
        wait_enter();
    }
}

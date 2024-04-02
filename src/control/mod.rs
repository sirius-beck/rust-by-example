mod conditionals;
mod loops;

use crate::utils::terminal::{clear_screen, show_menu, wait_enter};

pub fn execute() {
    loop {
        let items = [
            "Conditionals",
            "For - Range",
            "For - Array",
            "While",
            "Loop",
        ];
        let selected = show_menu("Control", &items, false);

        clear_screen();

        match selected {
            1 => conditionals::example(),
            2 => loops::example_for_range(),
            3 => loops::example_for_array(),
            4 => loops::example_while(),
            5 => loops::example_loop(),
            _ => break,
        }

        println!();
        wait_enter();
    }
}

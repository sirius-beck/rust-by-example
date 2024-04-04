mod control;
mod functions;
mod fundamentals;
mod ownership;
mod types;
mod utils;

use std::process::exit;

use utils::terminal::{clear_screen, show_menu};

fn main() {
    loop {
        let items = ["Fundamentals", "Types", "Control", "Functions", "Ownership"];
        let selected = show_menu("Main Menu", &items, true);

        clear_screen();

        match selected {
            1 => fundamentals::execute(),
            2 => types::execute(),
            3 => control::execute(),
            4 => functions::execute(),
            5 => ownership::execute(),
            _ => {
                println!();
                exit(0);
            }
        }
    }
}

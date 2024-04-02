mod r#move;
mod reference;
mod scope;
mod slice;

use crate::utils::terminal::{clear_screen, show_menu, wait_enter};

pub fn execute() {
    loop {
        let items = [
            "Basic",
            "Lifetime",
            "Move",
            "Clone",
            "Copy",
            "Moving Ownership #1",
            "Moving Ownership #2",
            "Immutable Reference",
            "Mutable Reference #1",
            "Mutable Reference #2",
            "Pending Reference",
            "Slice",
        ];
        let selected = show_menu("Ownership", &items, false);

        clear_screen();

        match selected {
            1 => scope::example_basic(),
            2 => scope::example_lifetime(),
            3 => scope::example_move(),
            4 => scope::example_clone(),
            5 => scope::example_copy(),
            6 => r#move::example_a(),
            7 => r#move::example_b(),
            8 => reference::example_immutable_ref(),
            9 => reference::example_mutable_ref_a(),
            10 => reference::example_mutable_ref_b(),
            11 => reference::example_pending_ref(),
            12 => slice::example(),
            _ => break,
        }

        println!();
        wait_enter();
    }
}

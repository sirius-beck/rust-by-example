use rpassword::prompt_password;
use std::io::Write;

pub fn show_menu(title: &str, items: &[&str], quit: bool) -> u32 {
    clear_screen();

    let full_title: String = String::from("Rust Examples :: ") + title;
    println!("{}", full_title);
    println!("{}", String::from("=").repeat(full_title.len()));

    show_menu_items(items);
    println!("{}", if quit { "* - Exit" } else { "* - Beck" });

    print!("\nChoose an option: ");
    std::io::stdout().flush().unwrap();

    let mut selected = String::new();
    std::io::stdin().read_line(&mut selected).unwrap();

    let option: Result<u32, _> = selected.trim().parse();

    match option {
        Ok(op) => op,
        _ => 0,
    }
}

pub fn show_menu_items(items: &[&str]) {
    for (i, item) in items.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn wait_enter() {
    prompt_password("Press ENTER to continue...").unwrap();
}

pub fn clear_screen() {
    print!("{}c", 27 as char);
}

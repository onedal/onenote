pub fn print_menu() {
    println!();
    println!("**** PROGRAM MENU ****");
    println!("Enter command:");
    println!("'show' - show all notes");
    println!("'add' - add new note");
    println!("'remove' - remove last note");
    println!("'quit' - close program");
    println!();
}

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn show_notes(notes: &Vec<String>) {
    println!("**** NOTES ****");
    for (i, note) in notes.iter().enumerate() {
        println!("{}: {}", i, note);
    }
}

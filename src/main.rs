mod notes;
mod engine;

use crate::notes::notes::Notes;


fn main() {
    let mut notes = Notes::new();

    loop {
        engine::print_menu();

        let command = engine::read_input();

        match command.trim(){
            "show" => engine::show_notes(&notes.list()),
            "add" => notes.add(engine::read_input()),
            "remove" => notes.remove(),
            "quit" => break,
            _ => println!("Invalid command"),
        }

    }
}






// #[cfg(test)]
// mod tests {

//     #[test]
//     fn print_menu() {
//         let note = "test note".to_string();
//         let mut notes = Vec::new();
//         notes.push(note);

//         assert_eq!(notes.len(), 1);
        
//     }

// }
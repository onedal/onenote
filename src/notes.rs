pub mod notes {
    pub struct Notes {
        data: Vec<String>,
    }

    impl Notes {
        pub fn new() -> Self {
            Self { data: vec![] }
        }

        pub fn add(&mut self, note: String) {
            self.data.push(note);
        }

        pub fn list(&self) -> Vec<String> {
            self.data.clone()
        }

        pub fn remove(&mut self) {
            self.data.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::notes::Notes;

    #[test]
    fn add_note() {
        let mut notes = Notes::new();

        let note = String::from("hello");
        notes.add(note.clone());

        assert_eq!(&note, notes.list().last().unwrap());
    }

    #[test]
    fn remove_note() {
        let mut notes = Notes::new();

        let note = String::from("hello");
        notes.add(note.clone());

        notes.remove();

        assert_eq!(notes.list().len(), 0);
    }

    #[test]
    fn show_notes() {
        let mut notes = Notes::new();

        let note = String::from("hello");
        notes.add(note.clone());

        assert_eq!(&note, notes.list().last().unwrap());
    }

    #[test]
    fn notes_len() {
        const COUNT: usize = 10;

        let mut notes = Notes::new();

        let initial_size = notes.list().len();

        for counter in 0..COUNT {
            notes.add(counter.to_string())
        }

        let notes_list = notes.list();

        assert_eq!(notes_list.len() - initial_size, COUNT);
    }
}

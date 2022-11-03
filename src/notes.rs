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


// #[cfg(test)]    // Компилируем этот модуль только при запуске тестов. В самом приложении он не нужен.
// mod tests {     // Модуль, содержащий тесты.
//     use super::notes::Notes;   // Используем тип notes из родительского модуля.

//     #[test]
//     fn add_note() {
//         // Создадим новый объект типа Notes.
//         let mut notes = Notes::new();

//         // Добавляем заметку.
//         let note = String::from("hello");
//         notes.add(note.clone());

//         // Проверяем, добавилась ли заметка.
//         assert_eq!(&note, notes.list().last().unwrap());
//     }

//     #[test]
//     fn notes_len() {
//         // Количество новых заметок
//         const COUNT: usize = 10;

//         let mut notes = Notes::new(path);

//         // Количество заметок до добавления новых.
//         let initial_size = notes.list().len();

//         // Для значений counter с 0 до COUNT ...
//         for counter in 0..COUNT {
//             // ... добвляем заметку - текстовое представление значения счётчика.
//             notes.add(counter.to_string())
//         }

//         let notes_list = notes.list();

//         // Проверяем, что добавилось ровно COUNT заметок.
//         assert_eq!(notes_list.len() - initial_size, COUNT);
//     }
// }
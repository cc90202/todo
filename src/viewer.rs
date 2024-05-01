//! # viewer crate
//!
//! `viewer` racchiude la gestion delle UI per i task e i todos.

pub mod ui
{
    use std::fmt;
    use console::Attribute;
    use console::style;
    use dialoguer::{Input, Select};
    use dialoguer::theme::ColorfulTheme;
    use crate::todo_core::logic;
    use crate::todo_core::logic::State;

    /// Viewer è la struttura con l'oggetto Todos da gestire graficamente
    pub struct Viewer {
        list: logic::Todos
    }

    
    impl Viewer  {
        /// new() implementa la creazione di un oggetto Viewer
        pub fn new() -> Viewer
        {
            Viewer{
                list: logic::Todos::new()
            }
        }

        /// init_db() crea, se non esiste, il db e inizializza.
        pub fn init_db(&mut self) -> rusqlite::Result<()>
        {
            self.list.init_db()  
        }
        
        /// main_loop() è il loop con le opzioni da scegliere via UI.
        pub fn main_loop(&mut self)
        {
            let quit = format!("{}", style("quit").attr(Attribute::Italic).green());
            let items = vec!["add", "done", "list", "clear", &quit];
            loop {

                let selection = Select::new()
                    .with_prompt("What do you choose?")
                    .items(&items)
                    .default(0)
                    .interact()
                    .unwrap();

                match items[selection] {
                    "add" => {self.add();},
                    "done" => {self.done()},
                    "list" => {self.list()},
                    "clear" => {self.clear()},
                    _ => break
                }
            }
        }

        // add() aggiunge il task ai todos
        pub fn add(&mut self)
        {

            let text = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Task:".to_string())
                .interact()
                .unwrap();
            self.list.add(logic::Task::new(text, logic::State::OnGoing));
        }

        /// list() elenca a video la lista di task  contenuta in todos
        pub fn list(&self)
        {
            println!("{}", self);
        }

        /// clear() cancella la lista dei task da todos
        pub fn clear(&mut self)
        {
            self.list.clear();
        }
        
        /// load() carica i task presenti nel db
        pub fn load(&mut self)
        {
            self.list.load();
        }

        // done() imposta allo stato Done un task
        pub fn done(&mut self)
        {
            // chiede il task attraverso la descrizione
            let msg = style("Task >").blue();

            let text = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt(msg.to_string())
                .interact()
                .unwrap();

            let task = logic::Task::new(text, logic::State::OnGoing);
            if let Some(t) = self.list.find_task(task)
            {
                t.set_status(logic::State::Done);
                let task = t.clone();
                self.list.set_done(task, State::Done);
            }
            else { println!("Task non trovato"); }
        }
    }

    /// Display() implementa il trait per il Viewer
    impl fmt::Display for Viewer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "List: {}", self.list).expect("TODO: panic message");
            Ok(())
        }
    }
    

} // end module
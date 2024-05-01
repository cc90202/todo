//! # todo_core crate
//!
//! `todo_core` racchiude la logica del task e della collezione di task.

pub mod logic
{
    use std::fmt;
    use console::{Attribute, style};
    use crate::todo_core::logic;
    use crate::todo_db::db;

    /// State descrive lo stato di un task
    /// Done = completato
    /// OnGoing = in corso

    #[derive(Copy, Clone)]
    pub enum State
    {
        Done,
        OnGoing,
    }

    /// Display() implementa il trait per il tipo State.
    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self  {
                State::Done => write!(f, "{}", "done"),
                State::OnGoing => write!(f, "{}", "on going"),
            }
        }
    }

    /// From() implementa la conversione da stringa a state
    impl From<&str> for State
    {
        fn from(item: &str) -> Self {
            let binding = item.to_lowercase();
            let lower = binding.as_str();
            match lower
            {
                "on going" => State::OnGoing,
                "done" => State::Done,
                _ => State::OnGoing,
            }
        }
    }
    

    /// Task è la struttura che modella un elemento della todolist.
    /// text: è una descrizione del task
    /// status: lo stato del task (Done, OnGoing)
    pub struct Task
    {
        text: String,
        status: State
    }

    /// new() ritorna un oggetto di tipo Task
    impl Task  {
        pub fn new(text: String, status:State) -> Task
        {
            Task {
                text,
                status
            }
        }

        /// text() getter
        pub fn text(&self) -> &str
        {
            &self.text
        }

        /// status() getter
        pub fn status(&self) -> &State
        {
            &self.status
        }

        /// set_text() setter
        pub fn set_text(&mut self, value: String)
        {
            self.text = value
        }

        /// set_status() setter
        pub fn set_status(&mut self, value: State)
        {
            self.status = value
        }
    }

    /// Display() implementa il trait per Task
    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let s = match self.status {
                logic::State::Done => format!("Task ({}, {})", self.text,  style(self.status()).attr(Attribute::Italic).strikethrough().red()),
                logic::State::OnGoing => format!("Task ({}, {})", self.text,  style(self.status()).attr(Attribute::Italic).green()),
            };

            write!(f, "{}",s)
        }
    }

    /// Clone() implementa il trait per Task
    impl Clone for Task {
        fn clone(&self) -> Self {
            Task::new(self.text.clone(), self.status)
        }
    }

    /// Todos è la struct con la collezione di task
    pub struct Todos
    {
        todos: Vec<Task>,
        database: db::DB,
    }

    impl Todos {
        /// new() ritorna un oggetto Todos
        pub fn new() -> Todos
        {
            Todos {
                todos: Vec::<Task>::new(),
                database: db::DB::new(),
            }
        }

        /// init_db() inizializza e crea, se non esiste, il db
        pub fn init_db(&mut self) -> rusqlite::Result<()>
        {
            self.database.create_db()
        }

        /// add() aggiunge un task alla collezione e al database
        pub fn add(&mut self, task: Task)
        {
            let task_clone = task.clone();
            self.todos.push(task);

            match self.database.add_task(task_clone) {
                Ok(_) => {println!("Inserted");},
                Err(x)=> {eprintln!("Errore in inserimento: {}", x);}
            }
        }

        /// set_done() imposta lo stato (Done, OnGoing) e persiste sul db
        pub fn set_done(&mut self, task: Task, value: State)
        {
            // cerca task nella collection
            let result = self.todos.iter_mut().find(|x| x.text == task.text);
            match result
            {
                Some(x) => x.set_status(value),
                None => {println!("Task non presente: {}", task)}
            }
            match self.database.set_done(task, value) {
                Ok(_) => {println!("set done");},
                Err(x)=> {eprintln!("Errore in update stato: {}", x);}
            }
        }

        /// remove() rimuove un task dalla collezione
        pub fn remove(&mut self, task: Task)
        {
            // cerca task nella collection, alternativo a match
            if let Some(index) = self.todos.iter().position(|x| x.text == task.text) {
                self.todos.swap_remove(index);
            }
        }

        /// at() ritorna un elemento della collezione all'indice indicato
        pub fn at(&self, index: usize) -> Option<&Task>
        {
            if index < self.todos.len()
            {
                return self.todos.get(index);
                //return match self.todos.get(index) {
                 //   task => task,
                //}
            }
            None
        }

        /// find_task cerca task nella collezione
        pub fn find_task(&mut self, task: Task)-> Option<&mut Task>
        {
            let result = self.todos.iter_mut().find(|x| x.text == task.text);
            match result {
                Some(x) => Some(x),
                None => None,
            }
        }

        /// clear() cancella la collezione di task
        pub fn clear(&mut self)
        {
            self.todos.clear();
            match self.database.clear()
            {
                Ok(_) => println!("Lista cancellata"),
                Err(e) => eprintln!("Errore: {}", e),
            }
        }

        /// load() carica dal db i task
        pub fn load(&mut self)
        {
            match self.database.load(&mut self.todos) {
                Ok(_) => println!("Lista caricata"),
                Err(x) => eprintln!("Errore {}", x),
            }
        }
    }

    /// Dispay() implementa il trait per Todos
    impl fmt::Display for Todos {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            for item in &self.todos
            {
                write!(f, "({}) ", item).expect("TODO: panic message");
            }
            Ok(())
        }
    }

}   // end module
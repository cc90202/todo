//! # viewer db
//!
//! `db` racchiude la gestion della persistenza su db per i task e i todos.


pub mod db
{
    use std::env;
    use rusqlite::{Connection, Result};
    use crate::todo_core::logic;
    use crate::todo_core::logic::{State, Task};

    /// Struct DB modella la struttura per gestire il database.
    pub struct DB
    {
        db_path: String,
    }
    
    impl DB
    {
        /// create_db() crea, se non esiste, il database. 
        /// Questo metodo deve essere sempre chiamato per primo, inizializzando il path del db
        pub fn create_db(& mut self) -> Result<()>
        {
            //let mut db_path: String = "".to_string();
            if let Some(x) = env::current_dir().unwrap().to_str()
            {
                self.db_path = x.to_string() + "\\todo_list.db3"; 
                println!("creo il db in {}", self.db_path);
            }

            let conn = Connection::open(&self.db_path)?;
            let r = conn.execute(
                "CREATE TABLE IF NOT EXISTS todolist (
                    name TEXT NOT NULL PRIMARY KEY,
                    state TEXT NOT NULL
                )",
                (), // empty list of parameters.
            );

            match r {
                Ok(_) => println!("DB creato con successo o esistente in {}", conn.path().unwrap_or("")),
                Err(e)=> {eprintln!("Errore in creazione db: {}", e); return Err(e);}
            }
            Ok(())
        }
        
        /// new() crea un oggetto DB
        pub fn new() -> Self
        {
            DB
            {
                db_path: String::from("")
            }
        }

        /// add_task() aggiunge un task al db
        pub fn add_task(& self, task:logic::Task) -> Result<()>
        {
            println!("add: {}", &self.db_path);
            
            let conn = Connection::open(&self.db_path)?;

            conn.execute(
                "INSERT INTO todolist (name, state) VALUES (?1, ?2)",
                (&task.text(), &task.status().to_string()),
            )?;
            Ok(())
        }
        
        /// set_done() imposta lo stato del task
        pub fn set_done(&mut self, task: Task, value: State) -> Result<()>
        {
            let conn = Connection::open(&self.db_path)?;

            conn.execute(
                "UPDATE todolist SET state = ?1 WHERE name = ?2",
                (value.to_string(), &task.text()),
            )?;
            Ok(())
        }
        
        /// load() carica la lista dei task in memoria
        pub fn load(&mut self, todos :&mut Vec<Task>) -> Result<()>
        {
            let conn = Connection::open(&self.db_path)?;

            let mut stmt = conn.prepare("SELECT * FROM todolist")?;
            let task_iter = stmt.query_map([], |row| {
                let text = row.get(0)?;
                let status_str:String  = row.get(1)?;
                let status = match status_str.as_str() {
                        "on going" => State::OnGoing,
                        _ => State::Done
                };
                let task = Task::new(text, status);
                Ok(task)
            })?;

            for task in task_iter {
                todos.push(task.unwrap())
            }
            
            Ok(())
        }
        
        /// clear() cancella tutti i task dal database
        pub fn clear(&self) -> Result<()>
        {
            let conn = Connection::open(&self.db_path)?;

            conn.execute(
                "DELETE FROM todolist",
                (),
            )?;
            Ok(())
        }
    
    }
    


    
}   // end module


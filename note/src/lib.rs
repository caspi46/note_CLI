use clap::{Parser, Subcommand};

#[derive(Parser)] 
#[command(name = "tasker", version, about = "A CLI task manager")]
pub struct Note {
    #[command(subcommand)]
    pub command: Commands
}

impl Note {
    pub fn add(&self, new_note: &String, due_date: &String) {
        let note = Entry::new(String::from(new_note), String::from(due_date)); 
        note.add_result();
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
       #[arg(help = "New note name")]
        new_note: String, 
        due_date: String, 
    }, 

    Hi {
        #[arg(help = "Name")]
        name: String, 
    }
}

pub struct Entry {
    pub name: String, 
    pub due: String, // will change Date Object  
}

impl Entry {
    pub fn new(name: String, due: String) -> Self {
        Self {
            name,
            due,
        }
    }
    pub fn add_result(&self) {
        println!("New note: {} \nDue date: {}", self.name, self.due); 
    }
}

pub struct Notes {
    pub notes : vec<String>, 
    pub note_path : String, 
    pub note_bak : String, 
}
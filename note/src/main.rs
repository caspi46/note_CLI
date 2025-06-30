
use note::{Note, Commands};
use clap::{Parser}; 

// Planner note 
// 
fn main () {
    let com = Note::parse(); 
    match com.command {
        Commands::Add {ref new_note, ref due_date} => com.add(new_note, due_date),
        Commands::Hi { name } => println!("hello, {}", name),
    }
}
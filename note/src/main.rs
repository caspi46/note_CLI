
use note::{Note, Commands};
use regex::Regex;
use clap::{Parser}; 

// Planner note 
// 
pub fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let month = parts[0].parse().ok()?;
    let day = parts[1].parse().ok()?;
    let year = parts[2].parse().ok()?; 

    Some((year, month, day))
}

fn main () {
    let com = Note::parse(); 
    let date_format = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{2})$").unwrap(); 
    match com.command {
        Commands::Add {ref new_note, ref start, ref due} => {
            if !date_format.is_match(start) || !date_format.is_match(due) {
                panic!("Error! Date format must be: nn/nn/yy")
            }

            let start_date = parse_date(&start); 
            let due_date = parse_date(&due); 
            if start_date > due_date{
                panic!("Error! Start must be less than due date")
            }

            com.add(new_note, start_date, due_date);
        },
        Commands::Rm { ref note } => {
            println!("Rmove {}", note);
        }
    };
}
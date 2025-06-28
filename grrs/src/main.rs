// Goal: grrs foobar test.txt 
// #1 look at test.txt
// #2 print out the lines that contain "foobar" 

// Parsing CLI argument with Clap
use clap::Parser; 

/// Search for a pattern in a file and display the lines that contain it 
#[derive(Parser)]
struct Cli {
    /// The pattern to look for 
    pattern: String,
    /// The path to the file to read 
    path: std::path::PathBuf,
}


// Note: 
// There are a lot of custom attributes you can add to fields. 
// EX) 
// if want to use this field for the argument after -o or --output, 
// #[arg(short = 'o', long = "output")]


fn main() {
    let args = Cli::parse(); 

    let content = std::fs::read_to_string(&args.path).expect("couldn't read file"); 

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line); 
        }
    }
}
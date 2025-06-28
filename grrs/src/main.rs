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


fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let args = Cli::parse(); 

    // let content = std::fs::read_to_string(&args.path).expect("couldn't read file"); 

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line); 
    //     }
    // }

    // let result = std::fs::read_to_string("test.txt"); 
    // let content = match result {
    //     Ok(content) => { content }, 
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    // };
    // println!("file content: {}", content); 

    // OR fo short cut the above error handling => abort the program if error occurs 
    // let content = std::fs::read_to_string("text.txt").unwrap(); 

    // w/o panic => return the result<(), std::error::Error
    // let result = std::fs:read_to_string("test.txt"); 
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { return Err(error.into()); }
    // };
    // println!("file content: {}", content); 
    // Ok(()) // the result is OK, good to go.

    // Question Mark => just like .unwrap() but return in the error arm
    let content = std::fs::read_to_string("test.txt")?; // ? => convert to error type from std::io::Error
    println!("file content: {}", content);
    Ok(())
}
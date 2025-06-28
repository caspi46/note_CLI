// Goal: grrs foobar test.txt 
// #1 look at test.txt
// #2 print out the lines that contain "foobar" 

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given"); 
    let path = std::env::args().nth(2).expect("no path given"); 

    let args = Cli { 
        pattern: pattern.clone(),
        path: std::path::PathBuf::from(path.clone()),
    };

    println!("pattern: {:?}, path: {:?}", pattern, path);
}
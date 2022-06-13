// import libraries
use std::fs;
use std::io;

fn main() {
    // init the input
    let filename = "test.txt";

    // input some string
    let mut line: String = String::new();
    println!("Enter some string:");
    io::stdin().read_line(&mut line).unwrap();
    line.pop();
    println!("{}", line);

    // reading contents from the text file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    //check and count occurences
    let c = contents.matches(&line).count();

    println!("{}", c)

}
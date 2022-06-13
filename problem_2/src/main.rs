
use std::fs;

fn main() {
    // --snip--
    let filename = "test.txt";
    let word = "helloworld";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    let c = contents.matches(word).count();

    println!("{}", c)

}
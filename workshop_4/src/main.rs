
// Problem 1: fibonnaci number

// #[derive(Debug)]
// struct Fibonacci {
//     a: u32,
//     b: u32,
// }
// fn fibonacci_number () -> Fibonacci {
//     Fibonacci { a: 0, b: 1 }
// }

// impl Iterator for Fibonacci {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         let new_b = self.a + self.b;

//         self.a = self.b;
//         self.b = new_b;

//         Some(self.a)
//     }
    
// }
// fn main() {
//     for i in fibonacci_number().take(10) {
//         println!("> {}", i);
//     }
    
// }

//Problem 2: Lifetime error

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl fmt::Display for StrDisplayable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;

        }
        Ok(())
    }
} 

fn main() {
    let vec: Vec<&str> = vec!["a", "bc", "def"];
    let vec_foo = StrDisplayable(vec);
    println!("{}", vec_foo);
}


#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}
fn fibonacci_number () -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_b = self.a + self.b;

        self.a = self.b;
        self.b = new_b;

        Some(self.a)
    }
    
}
fn main() {
    for i in fibonacci_number().take(10) {
        println!("> {}", i);
    }
    
}

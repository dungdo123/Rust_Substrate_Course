// fn main() {
//     println!("Hello, world!");
// }
// Exercise 1:
// fn main() {
//     let x = change_value(10,&mut 20);
//     println!("{}",x)
// }

// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input == 1 {
//         *output = 3
//     }
//     else {
//         *output = 4
//     }
//     *output
// }

// Exersice 2:

// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime (num, &primes){
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool{
//     for &i in p {
//         if num > i && num % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// Exersice 3:

// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;
//     let mut max = 0;
//     for n in &mut *v{
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     for n in v{
//         *n = 100*(*n)/max;
//     }
//     println!("values: {:#?}", values);
// }

// Exercise 4:
fn main() {
    let mut a  = vec![1,2,3,4,5];
    
    let (a,c) = test(&mut a);
    println!("sum of all number is {}",c);
    println!("inversed vector is {:?}", a)
    
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8> = Vec::new();
    let mut c:u8=0;
    loop {
        if a.len() == 0 {break;}
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b,c as i32)
}
// // fn main() {
// //     println!("Hello, world!");
// // }
// // Struct:
// struct Object {
//     width: i32,
//     height: i32,
// }

// impl Object {
//     fn new(width: i32, height: i32) -> Object {
//         Object { width: width, height: height }
//     }
//     fn area(&self) -> i32 {
//         self.width* self.height
//     }
//     fn increase(&mut self) {
//         self.width = self.width + 20;
//         self.height = self.height + 20;
//     }
// }

// fn main(){
//     let mut p = Object {
//         width: 50,
//         height: 50,
//     };
//     println!("area of rectangular width {} and height {} is {}", p.width, p.height, p.area());
//     p.increase();
//     println!("area of rectangular width {} and height {} is {}", p.width, p.height, p.area());
// }

use std::{collections::HashMap};

// exercises
pub struct School<T> {
    students: HashMap<String, T>,
}
impl<T: std::fmt::Debug+ std::cmp::PartialEq + std::cmp::Ord> School<T> where T:Copy{
    pub fn new() -> Self {
        Self { students: HashMap::new() }
    }
    pub fn add(&mut self, student: &str, grade: T) {
        self.students.insert(String::from(student), grade);
    }
    pub fn print_std_list(&self) {
        for (s, g) in &self.students {
            println!("{} {:?}", s, g);
        }
    }
    pub fn grade_list(&self) {
        let mut grades: Vec<T> = Vec::new();
        for (_s, g) in &self.students {
            if !grades.contains(&g) {
                grades.push(*g);
            }
        }
        grades.sort();
        println!("list of grades is {:?}", grades);
    }
    pub fn find_student(&self, common_grade: T){
        let mut students_same_grade: Vec<String> = Vec::new();
        for (s, g) in &self.students {
            if *g == common_grade {
                students_same_grade.push(s.to_string());
            }
        }
        students_same_grade.sort_by(|a,b| a.to_lowercase().cmp(&b.to_lowercase()));
        println!("students have {:?} : {:?}", common_grade, students_same_grade);
    }
    
}

fn main () {
    let mut new_school = School::new();
    new_school.add("david", 9);
    new_school.add("alice", 9);
    new_school.add("bob", 4);
    new_school.print_std_list();
    new_school.grade_list();
    new_school.find_student(9);

}
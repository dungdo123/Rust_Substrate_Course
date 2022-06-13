
// problem 1: checking sub array

fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10];
    let mut count: i32 = 0;
    for i in org_arr{
        for j in sub_arr {
            if i == j {
                count += 1;
            }
        }
    }
    if count == (sub_arr.len() as i32) {
        println!("sub array found!");
    }
    else {
        println!("not matched!")
    }
}
use rustils::myutils;


pub fn main() {
    let x : Vec<u64> = std::ops::Range { start: 0, end: 10000 }.collect();
    let y : Vec<u64> = std::ops::Range { start: 0, end: 10000 }.collect();
    
    let result = myutils::multi_and_sum(x, y).unwrap();
    println!("{}", result)
}
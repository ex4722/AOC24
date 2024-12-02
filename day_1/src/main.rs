use std::fs;

fn main() {
    let contents: String =  fs::read_to_string("./input.txt")
        .expect("File open error");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in lines{
        let parsed: Vec<&str> = line.split("   ").collect();
        if parsed[0].is_empty(){
            break;
        }
        list1.push(parsed[0].parse().expect("Should be a int"));
        list2.push(parsed[1].parse().expect("Should be a int"));
    }
    list1.sort();
    list2.sort();

    let mut total_distance: i32 = 0;
    for i in 0..list1.len(){
        println!("{} and {}", list1[i], list2[i]);
        total_distance += (list1[i] - list2[i]).abs();
    }
    println!("Total Distance: {}", total_distance);
}

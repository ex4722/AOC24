use std::fs;

fn part1(){
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
        total_distance += (list1[i] - list2[i]).abs();
    }
    println!("Total Distance: {}", total_distance);
}

fn part2(){

    let contents: String =  fs::read_to_string("./input.txt")
        .expect("File open error");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in lines{
        let parsed: Vec<&str> = line.split("   ").collect();
        if parsed[0].is_empty(){
            break;
        }
        list1.push(parsed[0].parse().expect("Should be a int"));
        list2.push(parsed[1].parse().expect("Should be a int"));
    }
    let mut sim_score: usize= 0;
    for num in list1{
        // look inside list2 to find count
        let count:usize = list2.iter().filter(|&val| *val == num).count();
        sim_score += count * num;
    }
    println!("Sim Score: {}", sim_score);
}

fn main() {
    part1();
    part2();
}

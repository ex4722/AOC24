use std::fs;
use regex::Regex;

fn do_capture(re: Regex, contents: &String) -> i64{
    let mut multiplexer: i64 = 0;
    let mut do_multi: bool = true;
    for caps in re.captures_iter(&contents){
        match caps.get(0).unwrap().as_str(){
            "do()" => {
                do_multi = true; 
                continue;
            }
            "don't()" => {
                do_multi = false;
                continue;
            }
            _ => {
            }
        }
        if !do_multi{
            continue;
        }
        multiplexer += caps.get(1).unwrap().as_str().parse::<i64>().unwrap() * caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    }
    multiplexer
}
fn main() {
    let re1 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let contents = fs::read_to_string("./input.txt")
        .expect("File not found");

    println!("{}", do_capture(re1, &contents));
    println!("{}", do_capture(re2, &contents));
}

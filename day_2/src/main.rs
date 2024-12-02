use std::fs;

fn main() {
    let mut safe_counter:i32 = 0;
    let contents: String =  fs::read_to_string("./input.txt")
    .expect("File open error");
    let mut game: Vec<Vec<i64>> = Vec::new();

    game = contents.lines()
        .map(|line| {
            line.split_whitespace()
            .map(|x| {x.parse::<i64>().unwrap()})
            .collect::<Vec<i64>>()
        })
        .collect();

    for levels in game{
        let mut prev: i64 = levels[0];
        let mut increasing: bool = levels[0] < levels[1];
        for idx in 1..(levels.len()){
            if prev < levels[idx] && !increasing{
                break;
            }
            if prev > levels[idx] && increasing{
                break;
            }

            let difference: i64 = (prev - levels[idx]).abs();
            if !(difference >= 1 && difference <= 3){
                break;
            }
            if idx == (levels.len() -1){
                // hit end 
                safe_counter+= 1;
            }
            prev = levels[idx];
        }
    }
    println!("{}", safe_counter);
}

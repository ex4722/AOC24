use std::fs;

fn _part1(){
    let mut safe_counter:i32 = 0;
    let contents: String =  fs::read_to_string("./input.txt")
        .expect("File open error");

    let game: Vec<Vec<i64>> = contents.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {x.parse::<i64>().unwrap()})
                .collect::<Vec<i64>>()
        })
        .collect();

    for level in game{
        let mut prev: i64 = level[0];
        let increasing: bool = level[0] < level[1];
        for idx in 1..(level.len()){
            if prev < level[idx] && !increasing{
                break;
            }
            if prev > level[idx] && increasing{
                break;
            }

            let difference: i64 = (prev - level[idx]).abs();
            if !(difference >= 1 && difference <= 3){
                break;
            }
            if idx == (level.len() -1){
                // hit end 
                safe_counter+= 1;
            }
            prev = level[idx];
        }
    }
    println!("{}", safe_counter);
}


fn process_line(level: &Vec<i64>) -> (bool, usize){
    let increasing: bool = level[0] < level[1];

    for idx in 0..(level.len() - 1){
        if level[idx] == level[idx+1]{
            return (false,idx);
        }
        if level[idx] > level[idx+1] && increasing{
            return (false, idx);
        }
        if level[idx] < level[idx+1] && !increasing{
            return (false, idx);
        }

        let difference: i64 = (level[idx + 1] - level[idx]).abs();
        if !(difference >= 1 && difference <= 3){
            return (false, idx);
        }
    }
    return (true,0);
}

fn main() {
    let mut safe_counter:i32 = 0;
    let contents: String =  fs::read_to_string("./input.txt")
        .expect("File open error");

    let game: Vec<Vec<i64>> = contents.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {x.parse::<i64>().unwrap()})
                .collect::<Vec<i64>>()
        })
        .collect();

    for level in game{
        //println!("Line: {:?}", level);
        let (mut pass, error_idx) = process_line(&level);
        if pass{
            //println!("  Safe: {:?}", level);
            safe_counter += 1;
        }else{
            // didnt' pass, let's try again after removing until the end 
            for idx in (error_idx-1)..level.len(){
                let mut damper = level.clone();
                damper.remove(idx);
                //println!("Trying again after removing {} from {:?}", idx, damper);
                (pass, _) = process_line(&damper);
                if pass {
                    safe_counter += 1;
                    break;
                }
            }
        };
    };
    println!("{}", safe_counter);
}

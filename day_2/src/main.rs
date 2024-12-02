use std::fs;

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
    let mut damper_saves: i32 = 0;

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
        let (mut pass, error_idx) = process_line(&level);
        if pass{
            safe_counter += 1;
        }else{
            // didnt' pass, let's try again after removing until the end 
            for idx in (if error_idx!=0 {error_idx - 1} else {0} )..level.len(){
                let mut damper = level.clone();
                damper.remove(idx);
                (pass, _) = process_line(&damper);
                if pass {
                    damper_saves += 1;
                    break;
                }
            }
        };
    };
    println!("{}", safe_counter);
    println!("{}", safe_counter + damper_saves);
}

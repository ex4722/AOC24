use std::fs;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
enum Direction{
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Clone)]
enum BoardPoint{
    Empty,
    Character(Direction),
    Obstacle,
    Traveled,
    WTF,
}

fn find_player(board: &Vec<Vec<BoardPoint>>) -> (usize, usize){
    let char: Vec<(usize,usize)> = board.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
            .enumerate()
            .filter_map(move |(y, value)| {
                if matches!(value, BoardPoint::Character(_)){
                        Some((x,y))
                    }else{
                        None
                    }
            })
        }).collect();
    *char.get(0).unwrap()
}

fn move_forward(board: &mut Vec<Vec<BoardPoint>>) -> bool {
    let (og_x, og_y): (usize, usize) = find_player(board);
    let (mut x,mut y): (usize, usize) = (og_x, og_y);

    let mut turn_direction: BoardPoint = BoardPoint::Character(Direction::West); 
    match board[og_x][og_y]{
        BoardPoint::Character(Direction::North) => {x -= 1; turn_direction = BoardPoint::Character(Direction::East)},
        BoardPoint::Character(Direction::East) =>  {y += 1; turn_direction = BoardPoint::Character(Direction::South)},
        BoardPoint::Character(Direction::South) => {x += 1; turn_direction = BoardPoint::Character(Direction::West)},
        BoardPoint::Character(Direction::West) =>  {y -= 1; turn_direction = BoardPoint::Character(Direction::North)},
        _ => println!("WTF NOT PLAYER"),
    }
    println!("Next point {},{}", og_x,og_y);
    // check bounds 
    if let Some(row) = board.get(x) {
        if let Some(_) = row.get(x) {
        }else{
            return false
        }
    }else{
        return false
    }

    if matches!(board[x][y], BoardPoint::Obstacle){
        // TURN
        println!("Found obstacle turning {:?}", turn_direction);
        board[og_x][og_y] = turn_direction;
    }else{
        // go forward
        //board[x][y] = board.get(og_x).unwrap().get(og_y).unwrap().clone();
        board[x][y] = board[og_x][og_y].clone(); // Clone the value
        board[og_x][og_y] = BoardPoint::Traveled;
    }
    true
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Error while opening file");
    let mut board: Vec<Vec<BoardPoint>> = contents.lines()
        .map(|line|{
            line.chars().map(|character|{
                match character{
                    '.' => BoardPoint::Empty,
                    '#' => BoardPoint::Obstacle,
                    '^' => BoardPoint::Character(Direction::North),
                    '>' => BoardPoint::Character(Direction::East),
                    'v'|'V' => BoardPoint::Character(Direction::South),
                    '<' => BoardPoint::Character(Direction::West),
                    _ => BoardPoint::WTF,
                }
            })
                .collect::<Vec<BoardPoint>>()
        }).collect();

    // loop until we hit a fuckin error
    // why do i need to specifc the ref is mutable?
    while move_forward(&mut board){
    }
    println!("DONE BITCH");
    let count = board.iter().flatten().filter(|value| {matches!(value, BoardPoint::Traveled)}).count();
    println!("{}", count + 1);
}

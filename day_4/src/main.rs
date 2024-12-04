use std::fs;


const LEN:usize= 3;

fn check_xmas_count(board: Vec<Vec<char>>, row: usize, col: usize) -> usize{
    let row_start = if row.saturating_sub(LEN) > 0 {row - LEN} else {0};
    let row_end   = if row + LEN < board.len() {row + LEN } else {board.len() - 1 };

    let col_start = if col.saturating_sub(LEN) > 0 {col - LEN } else {0};
    let col_end   = if col + LEN < board[0].len() {col + LEN } else {board[0].len() - 1};


    println!("{} and {} and {}", row_start, row, row_end);
    println!("{} and {} and {}", col_start, col, col_end);
    //
    println!("{} and {}", row, col);


    let mut perms: Vec<String> = Vec::new();
    // check forwards and backwards row
    perms.push(board.get(row)
        .and_then(|r| r.get(col_start..=col)
         .map(|slice| slice.iter().rev().collect::<String>()))
        .unwrap_or("".to_string()));

    perms.push(board.get(row).and_then(|r| r.get(col..=col_end).map(|slice| slice.iter().collect::<String>())).unwrap_or("".to_string()));

    //// check up and down
    perms.push((row..=row_end).map(|i| {
        board.get(i).unwrap().get(col).unwrap_or(&' ')
    })
    .copied()
    .collect::<String>());

    perms.push((row_start..=row).map(|i| {
        board.get(i).unwrap().get(col).unwrap_or(&' ')
    })
    .copied()
        .rev()
    .collect::<String>());

    // going the diagonal now
    perms.push((row..=row_end)
        .zip(col..=col_end)
        .map(|(row, col)|{
            board[row][col]
        })
        .collect::<String>());
    

    perms.push((row_start..=row).rev()
        .zip((col_start..=col).rev())
        .map(|(row, col)|{
            board[row][col]
        })
        .collect::<String>());

    perms.push((row..=row_end)
        .rev()
        .zip(col_start..=col)
        .map(|(row, col)|{
            board[row][col]
        })
        .collect::<String>()
        .chars()
        .rev()
                .collect::<String>()    // Collect back into a string

    );

    perms.push((row_start..=row).rev()
        .zip(col..=col_end)
        .map(|(row, col)|{
            board[row][col]
        })
        .collect::<String>());

    let count = perms.iter().filter(|x| x == &"XMAS").count();
    print!("{:?}", perms);
    print!("{}", count);
    count
}


fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File open error");

    let board: Vec<Vec<char>> = contents.lines().map(|line| {
        line.chars().collect()
    }).collect();

    //let total_count = (0..board.len()-1).
    //map(0..board[0].len()-1)

    let mut total_count= 0;

    for row_idx in 0..board.len() {
        for col_idx in 0..board[row_idx].len() {
            total_count += check_xmas_count(board.clone(), row_idx, col_idx);
        }
    }



    //println!("{}", check_xmas_count(board.clone(), 3, 4));
    println!("\n{}", total_count);

    // search for each X value and then get the count of XMAS
}

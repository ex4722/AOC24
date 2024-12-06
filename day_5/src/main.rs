use std::fs;
use std::collections::HashMap;


fn check_entry_valid(page: &Vec<usize>, idx: usize, mut page_order_hashmap: HashMap<usize, Vec<usize>>) -> bool{
        for prev in &page[0..idx]{
            if page_order_hashmap.entry(*page.get(idx).unwrap()).or_default().contains(prev){
                return false;
            }
        }
    return true;
}

fn check_page_valid(page: Vec<usize>, mut page_order_hashmap: HashMap<usize, Vec<usize>>) -> bool{
    for (idx, value) in page.iter().enumerate(){
        // check if any value that should be after shows up before
        for prev in &page[0..idx]{
            if page_order_hashmap.entry(*value).or_default().contains(prev){
                return false;
            }
        }
    }
    return true;
}

fn _part_1(){
    let contents = fs::read_to_string("./test.txt").expect(
        "Can't open file"
    );

    let mut page_order_hashmap: HashMap<usize, Vec<usize>> = HashMap::new();
    contents.split("\n\n")
        .nth(0).unwrap().lines()
        .for_each(|line| {
            let mut parts = line.split('|').map(|x| x.parse::<usize>().unwrap());
            page_order_hashmap.entry(parts.next().unwrap()).or_insert_with(|| vec!()).push(parts.next().unwrap());
        });
    
    // updates here
    let middle_values: usize = contents.split("\n\n")
        .nth(1).unwrap().lines()
        .map(|update|{
            update.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        })
        .filter(|processed_update| {
           check_page_valid(processed_update.clone(), page_order_hashmap.clone())
        })
        .map(|valid_update| *valid_update.get(valid_update.len()/ 2).unwrap()).sum();
        // wtf is the difference?
        //.map(|valid_update| valid_update.get(valid_update.len()/ 2).unwrap()).sum();
    println!("{}", middle_values);
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect(
        "Can't open file"
    );

    let mut page_order_hashmap: HashMap<usize, Vec<usize>> = HashMap::new();
    contents.split("\n\n")
        .nth(0).unwrap().lines()
        .for_each(|line| {
            let mut parts = line.split('|').map(|x| x.parse::<usize>().unwrap());
            page_order_hashmap.entry(parts.next().unwrap()).or_insert_with(|| vec!()).push(parts.next().unwrap());
        });
    println!("{:?}", page_order_hashmap);

    let mut invalid_middle_count = 0; 
    // find fucked arrays and unfuck 
    contents.split("\n\n")
        .nth(1).unwrap().lines()
        .map(|update|{
            update.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        })
        .filter(|processed_update| {
           !check_page_valid(processed_update.clone(), page_order_hashmap.clone())
        })
        .for_each(|mut invalid_page|{
            // start backwards and look forward
            while !check_page_valid(invalid_page.clone(), page_order_hashmap.clone()){

            (0..invalid_page.len()).rev().for_each(|idx| {
                if !check_entry_valid(&invalid_page, idx, page_order_hashmap.clone()){
                    // entry not valid so move it backwards, loop over page_order to find first
                    // value's index
                    let mut replace_idx = 0;
                    for i in 0..idx{
                        //println!("Checking if {} is in {:?} idx {}", invalid_page.get(i).unwrap(), page_order_hashmap.get(&idx).unwrap_or(&vec![]), invalid_page[idx]);
                        if page_order_hashmap.get(&invalid_page[idx]).unwrap_or(&vec![]).contains(invalid_page.get(i).unwrap()){
                            replace_idx = i;
                            invalid_page.swap(replace_idx, idx);
                            //println!("  swapping {} and {}", replace_idx, idx);
                            break;
                        }
                    }
                    // do swap here and current entry should be valid 
                }else{
                }
                });
            }


            println!("{:?}", invalid_page);
            invalid_middle_count += invalid_page.get(invalid_page.len()/ 2).unwrap();
            if !check_page_valid(invalid_page, page_order_hashmap.clone()){
                println!("WTF");
            };

        });
        println!("{:?}", invalid_middle_count);
}

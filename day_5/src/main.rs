use std::fs;
use std::collections::HashMap;


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

    //let page_order_files: Vec<(usize,usize)> = contents.split("\n\n")
    //    .nth(0).unwrap().lines()
    //    .map(|line| {
    //        let mut parts = line.split('|').map(|x| x.parse::<usize>().unwrap());
    //        (parts.next().unwrap(), parts.next().unwrap()) // Create a tuple from parsed numbers
    //        //page_order_hashmap.entry(parts.next().unwrap()).or_insert_with(|| !vec(parts.next().unwrap()))
    //    })
    //    .collect();
    //println!("{:?}", page_order_files);


}

use std::fs;

fn main() {
    println!("Reading input file...");

    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];

    // Part 1
    let contents = fs::read_to_string("input.txt")
                .expect("Should have been able to read the file input.txt");


    let mut elves: Vec<u32> = 
            contents.split("\n\r\n")
            .map(|elf| elf.lines())
            .map(|food_items| food_items.map(|item| item.parse::<u32>().unwrap()).sum())
            .collect();
    
    elves.sort();
    elves.reverse();

    // OLD WAY
    // let biggest: u32 = 
    //         elves.iter()
    //         .map(|item| *item)
    //         .max().unwrap_or(0);
    
    println!("Top: {}", elves[0]);


    // Part 2
    print!("Top 3: {}",elves[0] + elves[1] + elves[2]);
            
}

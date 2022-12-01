use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn day_one() {
    let fileName = "input/dayOneInput.txt";
    let file = File::open(fileName).unwrap();
    let reader = BufReader::new(file);

    let mut currentHigh = -1; 
    let mut groupCount: i32 = 0; 

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line == "\r" || line.is_empty() {
            if groupCount > currentHigh {
                currentHigh = groupCount; 
            }
            groupCount = 0; 
        }
        else { 
            groupCount += line.parse::<i32>().unwrap();
        }  
    }
    println!("{}", currentHigh);
    println!(""); 
    day_one_part_two()
}

fn check_top_three(leaders: &mut [i32; 3], check_value: i32) {
    if check_value < leaders [2] {
        return
    }
    else if check_value > leaders[0] {
        leaders[2] = leaders[1];
        leaders[1] = leaders[0]; 
        leaders[0] = check_value; 
    }
    else if check_value > leaders[1] {
        leaders[2] = leaders[1];
        leaders[1] = check_value; 
    }
    else {
        leaders[2] = check_value; 
    }



}

pub fn day_one_part_two() {
    let fileName = "input/dayOneInput.txt";
    let file = File::open(fileName).unwrap();
    let reader = BufReader::new(file);

    let mut leaders: [i32; 3] = [0; 3];
    let mut groupCount: i32 = 0; 

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line == "\r" || line.is_empty() {
            check_top_three(&mut leaders, groupCount); 
            groupCount = 0; 
        }
        else { 
            groupCount += line.parse::<i32>().unwrap();
        }  
    }
    
    println!("Day One Part Two Answer:"); 
    println!("Top Elf: {}", leaders[0]);
    println!("Second Elf: {}", leaders[1]);
    println!("Third Elf: {}", leaders[2]);
    println!("");
    println!("Top three total: {}", leaders[0] + leaders[1] + leaders[2]);
    println!("");

}
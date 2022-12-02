use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashMap;

pub fn day_two() {
    let fileName = "input/dayTwoInput.txt";
    let file = File::open(fileName).unwrap();
    let reader = BufReader::new(file);

    let mut firstTotal = 0; 
    let mut secondTotal = 0; 

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        firstTotal += score_map(&line);
        secondTotal += score_map_two(&line);
    }

    println!("Day 2 Part 1 Answer: ");
    println!("{}", firstTotal); 
    println!("");
    println!("Day 2 Part 2 Answer: ");
    println!("{}", secondTotal); 
    println!("");
    
}

fn score_map(line: &String) -> i32 {
    let mut scores = HashMap::new(); 

    scores.insert(String::from("A Y"), 8); 
    scores.insert(String::from("A X"), 4); 
    scores.insert(String::from("A Z"), 3); 

    scores.insert(String::from("B Y"), 5); 
    scores.insert(String::from("B X"), 1); 
    scores.insert(String::from("B Z"), 9); 

    scores.insert(String::from("C Y"), 2); 
    scores.insert(String::from("C X"), 7); 
    scores.insert(String::from("C Z"), 6); 

    return scores.get(line).copied().unwrap_or(0);

}

fn score_map_two(line: &String) -> i32{
    let mut scores = HashMap::new();

    scores.insert(String::from("A Y"), 4); 
    scores.insert(String::from("A X"), 3); 
    scores.insert(String::from("A Z"), 8); 

    scores.insert(String::from("B Y"), 5); 
    scores.insert(String::from("B X"), 1); 
    scores.insert(String::from("B Z"), 9); 

    scores.insert(String::from("C Y"), 6); 
    scores.insert(String::from("C X"), 2); 
    scores.insert(String::from("C Z"), 7); 

    return scores.get(line).copied().unwrap_or(0);
}
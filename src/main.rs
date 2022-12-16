use std::{fs::File, io::{BufReader, BufRead}, path::Path, collections::HashMap};
use std::env;


// map to the solution functions for each day
#[path = "day01/solution.rs"] mod day01;
#[path = "day02/solution.rs"] mod day02;

fn load_solutions() -> HashMap<i32, fn(Vec<String>) -> String> {
    let mut solutions = HashMap::new();
    solutions.insert(1, day01::solve as fn(Vec<String>) -> String);
    solutions.insert(2, day02::solve as fn(Vec<String>) -> String);
    return solutions;
}

fn main() {
    println!("Lucien's AoC 2022!");
    // load solutions
    let solutions = load_solutions();
    // grab args
    let mut args = env::args();
    args.next(); // discard first arg
    // determine the day
    let day : i32 = match args.next() {
        None => panic!("Error: Please enter the day as a command line arg."),
        Some(day) => day.parse().unwrap(),
    };
    println!("Solving day {}", &day);
    // get the solution
    let solution : String = solutions[&day](
        lines_from_path(&format!(
            "src/day{}{}/input.txt",
            if day < 10 {"0"} else {""}, 
            &day))
    );
    // print the solution
    println!("{}", solution);
    // #TODO: Save to file
    return;
}

// Utility functions here:

pub fn lines_from_path(path: &String) -> Vec<String> {
    let path = Path::new(path);
    let file = match File::open(path) {
        Err(e) => panic!("File open error: {}", e),
        Ok(file) => file
    };
    let reader = BufReader::new(file);
    return reader.lines().map(|line| line.unwrap()).collect();
}

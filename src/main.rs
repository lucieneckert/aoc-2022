use std::{fs::File, io::{BufReader, BufRead}, path::Path};
use std::env;


// map to the solution functions for each day
#[path = "day1/solution.rs"] mod day1;

// const solutions : HashMap<i32, fn(Vec<String>) -> String> = HashMap::from([]);

fn main() {
    println!("Lucien's AoC 2022!");
    let mut args = env::args();
    args.next(); // discard first arg
    // determine the day
    let day = match args.next() {
        None => panic!("Error: Please enter the day as a command line arg."),
        Some(day) => day,
    };
    println!("Solving day {}", &day);
    // get the solution
    let solution : String = day1::solve(
        lines_from_path(&format!("src/day{}/input.txt", &day))
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

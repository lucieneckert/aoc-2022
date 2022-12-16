#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum RPSOptions {
  Rock = 1, // map to intrinsic value
  Paper = 2,
  Scissors = 3
}

use RPSOptions::*;

fn get_round_points (player : &RPSOptions, opponent : &RPSOptions) -> i32 {
  if *player == get_winning_choice(opponent) {return 6;}
  if *player == get_losing_choice(opponent) {return 0;}
  return 3;
}

fn get_winning_choice(against: &RPSOptions) -> RPSOptions {
  return match against {
    Rock => Paper,
    Paper => Scissors,
    Scissors => Rock
  }
}

fn get_losing_choice(against: &RPSOptions) -> RPSOptions {
  return match against {
    Rock => Scissors,
    Paper => Rock,
    Scissors => Paper
  }
}

pub fn solve(input: Vec<String>) -> String {
  let puzzle_to_solve = 2; // 1 or 2;
  // keep track of total score
  let mut total_score = 0;
  // iterate over input
  for round in input {
    let mut round_data = round.split(" ");
    // get the choices
    let opponent_choice : RPSOptions = match round_data.next().unwrap() {
      "A" => Rock,
      "B" => Paper,
      "C" => Scissors,
      _ => panic!("Error: Input is invalid, opponent choice not ABC")
    };
    let player_choice : RPSOptions = match round_data.next().unwrap() {
      "X" => if puzzle_to_solve == 1 {Rock} else {get_losing_choice(&opponent_choice)},
      "Y" => if puzzle_to_solve == 1 {Paper} else {opponent_choice},
      "Z" => if puzzle_to_solve == 1 {Scissors} else {get_winning_choice(&opponent_choice)},
      _ => panic!("Error: Input is invalid, player choice not XYZ")
    };
    // calculate round score and choice score
    let round_points = get_round_points(&player_choice, &opponent_choice);
    let choice_points = player_choice as i32;
    // add to total
    total_score += round_points + choice_points;
  }
  return total_score.to_string();
}
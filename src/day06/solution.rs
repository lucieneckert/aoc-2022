use std::collections::{HashSet, VecDeque};

const MARKER_LENGTH : usize = 14; // lmao I totally predicted part 2

pub fn solve(input: Vec<String>) -> String {
  // there's exactly one line of input
  let data = input.get(0).unwrap();
  let mut chars = data.chars();
  // keep track of a vector of the last MARKER_LENGTH chars
  let mut seen_chars : VecDeque<char> = VecDeque::new();
  // get the first MARKER_LENGTH chars
  for _ in 0..MARKER_LENGTH {
    seen_chars.push_back(chars.next().unwrap());
  }
  // now, check until we have a unique vector of seen chars
  // and return the index once we do
  let mut marker_idx = MARKER_LENGTH as i32;
  for next_char in chars {
    // println!("{:?}", seen_chars);
    let set_seen : HashSet<char> = seen_chars.iter().cloned().collect();
    // if we have all unique characters, we have the answer
    if set_seen.len() == seen_chars.len() {
      return marker_idx.to_string();
    }
    // otherwise, keep going
    seen_chars.pop_front();
    seen_chars.push_back(next_char);
    marker_idx += 1;
  }  
  return "Reached end of input, something went wrong.".to_string();
}
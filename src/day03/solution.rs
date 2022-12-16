use std::{collections::HashSet};

fn split_sack(sack: &String) -> [String; 2] {
  // string must be even len
  let split_index = sack.len() / 2;
  let first : String = sack[..split_index].to_string();
  let second : String = sack[split_index..].to_string();
  return [first, second];
}

fn get_priority(item: &char) -> i32 {
  // uppercase
  if (*item as i32) < ('a' as i32) {
    return *item as i32 - 'A' as i32 + 27; 
  } 
  // lowercase
  return *item as i32 - 'a' as i32 + 1;
}

const GROUP_SIZE : usize = 3;

pub fn solve(input: Vec<String>) -> String {
  let puzzle_to_solve = 2;
  let mut total_priority = 0;
  // track the groupsacks
  let mut groupsack : HashSet<char> = HashSet::new();
  for (idx, sack) in input.iter().enumerate() {
    if puzzle_to_solve == 1 {
      // split those sacks
      let [left, right] = split_sack(&sack);
      // track all left items and right items
      let mut left_items : HashSet<char> = HashSet::new();
      let mut right_items : HashSet<char> = HashSet::new();
      for item in left.chars() {
        left_items.insert(item);
      }
      for item in right.chars() {
        right_items.insert(item);
      }
      // find the intersection
      let shared_items = left_items.intersection(&right_items);
      // then sum the priorities
      let mut priority = 0;
      for item in shared_items {
        priority += get_priority(item);
      }
      // add to total, if puzzle 1
      total_priority += priority;
    } else { // groupsack time
      // see if we're starting a new groupsack
      let mut sack_items : HashSet<char> = HashSet::new();
      for item in sack.chars() {
        sack_items.insert(item);
      }
      if idx % GROUP_SIZE == 0 {
        groupsack = sack_items;
      } else {
        groupsack = groupsack
          .intersection(&sack_items) // get intersection
          .map(|c| *c) // deref
          .collect::<HashSet<char>>(); // slap that bad boy into a set
      }
      // if we're at the last idx in the group, we should have one item
      if idx % GROUP_SIZE == GROUP_SIZE - 1 {
        let badge = groupsack.iter().next().unwrap();
        let priority = get_priority(badge);
        total_priority += priority;
      }
    }
  }
  return total_priority.to_string();
}
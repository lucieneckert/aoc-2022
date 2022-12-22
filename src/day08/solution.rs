use std::{collections::HashSet};

const FOREST_SIZE : usize = 99;

fn parse_forest(input: Vec<String>) -> [[i32; FOREST_SIZE]; FOREST_SIZE] {
  let mut forest = [[0; FOREST_SIZE]; FOREST_SIZE];
  let mut y = 0;
  for line in input.iter() {
    let mut x = 0;
    for tree in line.chars().map(|c| c.to_digit(10).unwrap() as i32) {
      forest[y][x] = tree;
      x += 1;
    }
    y += 1;
  }
  return forest;
} 

fn count_in_direction(
  forest : [[i32; FOREST_SIZE]; FOREST_SIZE], 
  horizontal: bool,
  reverse: bool,
) -> HashSet<(usize, usize)>{
  let x_range = 0..FOREST_SIZE;
  let y_range = 0..FOREST_SIZE;
  // determine iteration order
  let a_iter = if horizontal {y_range.clone()} else {x_range.clone()};
  let b_iter = if horizontal {x_range.clone()} else {y_range.clone()};
  // track the tallest trees
  let mut tallest_tree : i32 = -1;
  let mut seen_trees = HashSet::new();
  for a in a_iter  {
    for mut b in b_iter.clone() {
      if reverse {
        b = (FOREST_SIZE - 1) - b;
      }
      let tree = forest[if horizontal {a} else {b}][if horizontal {b} else {a}];
      if tree > tallest_tree {
        seen_trees.insert(if horizontal {(a, b)} else {(b, a)});
        tallest_tree = tree;
      }
    }
    tallest_tree = -1;
  }
  // println!("Seen: {:?}. Total: {:?}", seen_trees, seen_trees.len());
  return seen_trees;
}

pub fn solve(input: Vec<String>) -> String {
  // track a set of visible coordinates, then count up at the end
  let mut seen_trees : HashSet<(usize, usize)> = HashSet::new();
  let forest = parse_forest(input);
  // count left to right
  let lr_trees = count_in_direction(forest, true, false);
  let rl_trees = count_in_direction(forest, true, true);
  let tb_trees = count_in_direction(forest, false, false);
  let bt_trees = count_in_direction(forest, false, true);
  // union all sets and count
  seen_trees.extend(lr_trees);
  seen_trees.extend(rl_trees);
  seen_trees.extend(tb_trees);
  seen_trees.extend(bt_trees);
  return seen_trees.len().to_string();
}
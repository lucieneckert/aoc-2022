use std::{collections::HashSet, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
  x: i32, 
  y: i32,
}

impl Position {

  fn is_far(&self, pos: Position) -> bool {
    let is_far_x = (self.x - pos.x).abs() > 1;
    let is_far_y = (self.y - pos.y).abs() > 1;
    return is_far_x || is_far_y;
  }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Position {
        Position {
          x: self.x + rhs.x,
          y: self.y + rhs.y
        }
    }
}

impl From<(i32, i32)> for Position {
  fn from(point: (i32, i32)) -> Position {
      Position { x: point.0, y: point.1 }
  }
}

fn apply_step(h_pos : Position, t_pos : Position, step : Position) -> (Position, Position) {
  let new_h_pos = h_pos + step;
  let mut new_t_pos = t_pos;
  if new_h_pos.is_far(t_pos) { // h was one away from t and moved to two away
    // figure out how t needs to move l/r, u/d
    if new_h_pos.y == t_pos.y {
      new_t_pos = Position::from(((new_h_pos.x + t_pos.x) / 2, t_pos.y));
    }
    else if new_h_pos.x == t_pos.x {
      new_t_pos = Position::from((t_pos.x, (new_h_pos.y + t_pos.y) / 2));
    }
    // otherwise, it's a diagonal  move away and t needs to go to h's old spot
    else {
      new_t_pos = h_pos;
    }
  }
  return (new_h_pos, new_t_pos);
}

pub fn solve(input: Vec<String>) -> String {
  let mut h_pos : Position = Position::from((0, 0));
  let mut t_pos : Position = Position::from((0, 0));
  // for puzzle 1, track sll t_pos
  let mut tail_visited : HashSet<Position> = HashSet::new(); 
  for line in input {
    let mut tokens = line.split(" ");
    let direction = tokens.next().unwrap();
    let num_steps : i32 = tokens.next().unwrap().parse().unwrap();
    // perform the input
    let step = Position::from(match direction {
      "U" => (0, 1),
      "D" => (0, -1),
      "L" => (-1, 0),
      "R" => (1, 0),
      _ => panic!("Bad input: No valid direction found")
    }); 
    for _ in 0..num_steps {
      (h_pos, t_pos) = apply_step(h_pos, t_pos, step);
      // track the tail position
      println!("h: {:?} t: {:?}", h_pos, t_pos);
      tail_visited.insert(t_pos);
    }
  }
  // we now have final positions
  println!("{:?}", tail_visited);
  return format!("1: {:?}", tail_visited.len());
}
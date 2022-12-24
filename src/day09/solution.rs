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

fn apply_step(rope : Vec<Position>, step : &Position) -> Vec<Position> {
  let mut new_rope : Vec<Position> = vec![];
  for (idx, segment) in rope.clone().iter().enumerate() {
    // if head
    if idx == 0 {
      // apply the step
      new_rope.push(segment.clone() + step.clone());
    } else {
      // otherwise, move according to new position of last segment
      let preceding_new_pos = new_rope.last().unwrap();
      let mut new_pos : Position = segment.clone();
      if segment.is_far(*preceding_new_pos) {
        //figure out how t needs to move l/r, u/d
        if preceding_new_pos.y == segment.y {
          new_pos = Position::from(((preceding_new_pos.x + segment.x) / 2, segment.y));
        }
        else if preceding_new_pos.x == segment.x {
          new_pos = Position::from((segment.x, (preceding_new_pos.y + segment.y) / 2));
        }
        // otherwise, move diagonally towards the preceding new pos
        else {
          new_pos = Position {
            x: if preceding_new_pos.x > segment.x {segment.x + 1} else {segment.x - 1},
            y: if preceding_new_pos.y > segment.y {segment.y + 1} else {segment.y - 1}
          }
        }
      }
      new_rope.push(new_pos);
    }
  }
  return new_rope;
}


#[allow(dead_code)]
fn print_rope(rope: &Vec<Position>) {
  let print_grid_size : i32 = 20;
  for y in -print_grid_size..print_grid_size {
    let mut line = String::new();
    for x in -print_grid_size..print_grid_size {
      line.push_str(
        if rope.contains(&Position { x, y })
          {"#"} 
        else 
          {"."}
      );
    }
    println!("{:?}", line);
  }
}

pub fn solve(input: Vec<String>) -> String {
  let mut rope : Vec<Position> = vec![Position::from((0, 0)); 10];
  // for puzzle 1, track all t_pos
  let mut tail_visited : HashSet<Position> = HashSet::new(); 
  for line in input {
    let mut tokens = line.split(" ");
    let direction = tokens.next().unwrap();
    let num_steps : i32 = tokens.next().unwrap().parse().unwrap();
    // perform the input
    let step = Position::from(match direction {
      "U" => (0, -1),
      "D" => (0, 1),
      "L" => (-1, 0),
      "R" => (1, 0),
      _ => panic!("Bad input: No valid direction found")
    }); 
    // println!("{:?}", line);
    for _ in 0..num_steps {
      rope = apply_step(rope, &step);
      // track the tail position
      tail_visited.insert(rope.last().unwrap().clone());
    }
    // print_rope(&rope);
  }
  // we now have final positions
  return format!("{:?}", tail_visited.len());
}
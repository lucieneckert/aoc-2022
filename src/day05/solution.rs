use crate::first_empty_line_idx;

const NUM_STACKS : usize = 9;
const PUZZLE_TO_SOLVE : i32 = 2;
#[derive(Debug)]
struct CrateStack {
  // stack order is [bottom, ..., top]
  stacks: Vec<Vec<char>>
}

impl CrateStack {

  // initializes the number of stacks we need
  pub fn init_stacks() -> CrateStack {
    let mut stacks = vec![];
    for _ in 0..NUM_STACKS {
      stacks.push(vec![]);
      print!("yeee");
    }
    return CrateStack { stacks: stacks };
  }

  // moves [num_crates] crates from [from] to [to] stacks.
  pub fn move_crates(&mut self, num_crates: i32, from: &usize, to: &usize) {
    // put all crates from the from stack into temp stack
    let from_stack = &mut self.stacks[*from];
    let mut temp_stack : Vec<char> = vec![];
    for _ in 0..num_crates {
      let moved_crate = from_stack.pop().unwrap();
      temp_stack.push(moved_crate);
    }
    // now, depending on puzzle, reverse the temp stack..
    if PUZZLE_TO_SOLVE == 2 {
      temp_stack.reverse();
    }
    // then place on the to stack
    for moved_crate in temp_stack {
      self.stacks[*to].push(moved_crate);
    }
    return;
  }

  // reverses all the stacks in the CrateStack
  pub fn reverse_stacks(&mut self) {
    self.stacks.iter_mut().for_each(|stack| stack.reverse());
  }

  // return the top chars of all stacks
  pub fn get_top_chars(&self) -> String {
    return self
      .stacks
      .iter()
      .map(|stack| match stack.len() {
        0 => " ".to_string(),
        n => stack[n - 1].to_string(),
      })
      .fold("".to_string(),|stack, acc|  stack + &acc);
  }
}

fn parse_cratestack(input: &Vec<String>) -> CrateStack {
  // parse the cratestack
  let mut crate_stack = CrateStack::init_stacks();
  for line in input {
    // stop when we find the blank one
    if line == "" {break}
    // use the length of the line to identify chars
    // they occur when idx % NUM_STACKS = 1
    for idx in 0..NUM_STACKS {
      let crate_idx = (idx * 4) + 1;
      let crate_char = line.chars().nth(crate_idx).unwrap();
      if crate_char.to_string() != " " {// no crate on this line
        crate_stack.stacks[idx].push(crate_char);
      }
    }
  }
  // we built it up from top->bottom, so reverse cols
  crate_stack.reverse_stacks(); 
  return crate_stack;
}

pub fn solve(input: Vec<String>) -> String {
  // create a crate_stack
  let mut crate_stack = parse_cratestack(&input);
  println!("{:?}", crate_stack.get_top_chars());
  // read out each instructions
  for line in &input[first_empty_line_idx(&input) + 1..] {
    println!("{}", &line);
    // get the three critical numbers: #crates, from, and to
    let mut nums = line
      .split(" ")
      .filter(
        |word| word.chars().all(
          |c| c.is_numeric()
        )
      )
      .map(|string_number| string_number.parse::<usize>().unwrap());
    // move the correct num crates
    let num_crates = nums.next().unwrap() as i32;
    let from = &nums.next().unwrap() - 1; // minus one bc 0-indexed
    let to = nums.next().unwrap() - 1;
    crate_stack.move_crates(num_crates, &from, &to);
  }
  // now we can see the final top of each stack
  println!("{:?}", crate_stack.get_top_chars());
  return "".to_string();
}

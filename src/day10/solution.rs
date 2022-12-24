// increments the cycle, and modifies interesting_signals if one is encountered
fn increment_cycle(cycle: i32, x : i32, interesting_signals: &mut Vec<i32>, screen: &mut [[char; 40]; 6]) -> i32 {
  // track the interesting signal
  if (cycle - 20) % 40 == 0 {
    interesting_signals.push(x * cycle);
  }
  // check to see if sprite is being rendered
  let pixel_x = if cycle % 40 - 1 == -1 {39} else{cycle % 40 - 1};
  let pixel_y = cycle / 40;
  if (pixel_x - x).abs() < 2 {
    screen[pixel_y as usize][pixel_x as usize] = '#';
  }
  return cycle + 1;
}

pub fn solve(input: Vec<String>) -> String {
  // iterate through input, tracking the cycle and the X register
  let mut x = 1;
  let mut cycle = 1;
  // puzzle 1: track the interesting signals
  let mut interesting_signals : Vec<i32> = vec![];
  // puzzle 2: render the screen
  let mut screen = [['.'; 40]; 6];
  for instruction in input {
    let mut tokens = instruction.split(" ");
    match tokens.next().unwrap() {
      "noop" => {
        // noop, increment cycle
        cycle = increment_cycle(cycle, x, &mut interesting_signals, &mut screen);
      },
      "addx" => {
        // increment cycles twice
        for _ in 0..2 {
          cycle = increment_cycle(cycle, x, &mut interesting_signals, &mut screen);
        }
        // set x
        x += tokens.next().unwrap().parse::<i32>().unwrap();
      },
      _ => panic!("Invalid instruction encountered"),
    }
  }
  //render screen
  screen.iter().for_each(
    |line| println!("{}", line.iter().fold(
      String::new(), |acc, pixel| format!("{}{}", acc, pixel))
    )
  );
  return String::from(
    format!(
      "1: {:?}", 
      interesting_signals.iter().sum::<i32>(), 
    )
  );
}
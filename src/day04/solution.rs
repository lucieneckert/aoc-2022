#[derive(Clone)]
#[derive(Copy)]
struct Assignment {
  lower: i32,
  upper: i32,
}

fn init_assignment(from: &String) -> Assignment {
  let mut bounds = from.split('-').map(|s| s.to_string());
  let lower : i32 = bounds.next().unwrap().parse().unwrap();
  let upper : i32 = bounds.next().unwrap().parse().unwrap();
  return Assignment {
    lower, upper
  };
}

fn len_assignment(ass: Assignment) -> usize {
  return (ass.upper - ass.lower) as usize;
}

pub fn solve(input: Vec<String>) -> String {
  let mut num_enclosed = 0;
  let mut num_overlapped = 0;
  for pairing in input {
    // get the two groups
    let mut groups = pairing.split(',');
    let first = init_assignment(&groups.next().unwrap().to_string());
    let second = init_assignment(&groups.next().unwrap().to_string());
    // find the bigger one
    let big : Assignment;
    let small : Assignment;
    if len_assignment(first) > len_assignment(second) {
      big = first;
      small = second;
    } else {
      big = second;
      small = first
    }
    // see if big encloses small
    num_enclosed += if big.upper >= small.upper && big.lower <= small.lower {1} else {0};
    // see if there's any overlap at all
    num_overlapped += if 
      (first.lower >= second.lower && first.lower <= second.upper) ||
      (second.lower >= first.lower && second.lower <= first.upper)
      {1} else {0}; 
  }
  return format!("1: {}, 2: {}", num_enclosed, num_overlapped);
}
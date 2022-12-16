const NUM_HIGHEST_CALORIES : usize = 3;

pub fn solve(input: Vec<String>) -> String {
  let mut highest_calories: [i32; NUM_HIGHEST_CALORIES + 1] = [0; NUM_HIGHEST_CALORIES + 1];
  let mut current_elf_calories: i32 = 0;
  for line in input {
    // break between elves
      if line == "" {
          for idx in 0..NUM_HIGHEST_CALORIES {
            // see if we've found its place
            let next_highest_calories = highest_calories[NUM_HIGHEST_CALORIES - idx - 1];
            if next_highest_calories > current_elf_calories {break};
            // otherwise, swap with the next highest
            highest_calories[NUM_HIGHEST_CALORIES - idx - 1] = current_elf_calories;
            highest_calories[NUM_HIGHEST_CALORIES - idx] = next_highest_calories;
          }
          current_elf_calories = 0;
      } else {
        let line_calorie_value : i32 = str::parse(&line).unwrap();
        current_elf_calories += line_calorie_value;
      }
  }
  // sum up the top n and print out
  return format!(
    "{:?}", 
    &highest_calories[0..NUM_HIGHEST_CALORIES]
      .iter()
      .sum::<i32>()
  );
}

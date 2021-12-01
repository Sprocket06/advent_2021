use std::fs;

fn main(){
  let input = fs::read_to_string("./day_1/input.txt").unwrap();
  let lines = input.lines().collect::<Vec<&str>>();
  let sections = lines.windows(2);
  
  let mut count = 0;
  for section in sections {
    if section[1].parse::<u32>().unwrap() > section[0].parse::<u32>().unwrap()
    {
      count += 1;
    }
  }
  println!("Measurements larger than previous: {}", count);
}

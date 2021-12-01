use std::fs;

fn main(){
  let input = fs::read_to_string("./day_1/input.txt").unwrap();
  let lines = input.lines().collect::<Vec<&str>>();
  let sections = lines.windows(3).collect::<Vec<_>>();
  
  let mut count = 0;
  for i in 1..sections.len() {
    let sec = sections[i].iter().map(|a| a.parse::<u32>().unwrap()).fold(0, |a, b| a + b);
    let prev_sec = sections[i-1].iter().map(|a| a.parse::<u32>().unwrap()).fold(0, |a, b| a + b);
    if sec > prev_sec {
      count += 1;
    }
  }
  println!("Measurements larger than previous: {}", count);
}

use std::fs;

fn main(){
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = input.lines();
  let mut depth = 0;
  let mut pos = 0;
  for line in lines {
    let bits = line.split(" ").collect::<Vec<&str>>();

    match bits[0] {
      "forward" => pos += bits[1].parse::<u32>().unwrap(),
      "up" => depth -= bits[1].parse::<u32>().unwrap(),
      "down" => depth += bits[1].parse::<u32>().unwrap(),

      _ => panic!("bad news bears")
    }
  }

  println!("{}", depth * pos)
}

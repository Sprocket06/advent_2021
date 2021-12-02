use std::fs;

fn main(){
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = input.lines().map(|item| item.split(" ").collect::<Vec<&str>>());
  let mut aim = 0;
  let mut depth = 0;
  let mut pos = 0;
  
  for line in lines {
    match line[0] {
      "up" => aim -= line[1].parse::<u32>().unwrap(),
      "down" => aim += line[1].parse::<u32>().unwrap(),
      "forward" => {
        let x = line[1].parse::<u32>().unwrap();
        pos += x;
        depth += aim * x;
      },

      _ => panic!("bad news bears")
    }
  }
  println!("{}", depth * pos);
}

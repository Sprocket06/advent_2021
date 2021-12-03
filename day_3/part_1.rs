use std::fs;

fn main(){
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = input.lines();
  let mut count: [i32; 12] = [0; 12];

  for line in lines {
    for (i, c) in line.chars().enumerate() {
      if c.to_digit(10).unwrap() > 0 {
        count[i] += 1;
      }else{
        count[i] -= 1;
      }
    }
  }

  let gamma = i32::from_str_radix(&count.map(|x| if x < 0 {'0'} else {'1'}).iter().collect::<String>(), 2).unwrap();
  let epsilon = i32::from_str_radix(&count.map(|x| if x < 0 {'1'} else {'0'}).iter().collect::<String>(), 2).unwrap();

  println!("gamma: {}\nepsilon: {}\ngamma * epsilon: {}", gamma, epsilon, gamma * epsilon)
}

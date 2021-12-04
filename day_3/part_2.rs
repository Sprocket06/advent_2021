use std::fs;

fn main(){
  let input = fs::read_to_string("input.txt").unwrap();
  let mut count = 0;
  let mut bit = 0;
  let mut oxygen_arr = input.lines().collect::<Vec<&str>>();
  let mut co2_arr = oxygen_arr.clone();
  
  while oxygen_arr.len() != 1 {
    let rm;
    for line in oxygen_arr.iter() {
      if line.chars().nth(bit) == Some('0') {
        count -= 1;
      }else{
        count += 1;
      }
    }

    if count >= 0 {
      rm = '0'
    }else{
      rm = '1'
    }

    oxygen_arr.retain(|x| x.chars().nth(bit).unwrap() != rm);
    bit += 1;
    count = 0;
  }

  bit = 0;
  count = 0;
  while co2_arr.len() != 1 {
    let rm;
    for line in co2_arr.iter() {
      if line.chars().nth(bit) == Some('0') {
        count -= 1;
      }else{
        count += 1;
      }
    }

    if count >= 0 {
      rm = '1'
    }else{
      rm = '0'
    }

    co2_arr.retain(|x| x.chars().nth(bit).unwrap() != rm);
    bit += 1;
    count = 0;
  } 

  let oxygen = u32::from_str_radix(oxygen_arr[0], 2).unwrap();
  let co2 = u32::from_str_radix(co2_arr[0], 2).unwrap();

  println!(
    "Oxygen: {}\nCO2: {}\nLife Support Rating: {}",
    oxygen, co2, oxygen * co2
  );
}

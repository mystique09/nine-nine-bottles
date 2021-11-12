use std::io;

fn sing(bottles: u32) {
  for i in (0..= bottles).rev() {
    println!("{}", read_bottles(i));
  }
}

fn read_bottles(bottles: u32) -> String {
  match bottles {
    2 => {
      format!("{} bottles of beer on the wall.\n{} bottles of beer. Take one down, pass it around.\n{} bottle of beer on the wall!\n", bottles, bottles, bottles- 1)
    },
    1 => {
      format!("{} bottle of beer on the wall.\n{} bottle of beer. Take one down, pass it around.\n{} bottles of beer on the wall!\n", bottles, bottles, bottles- 1)
    },
    0 => String::from("Let's do it again!\n"),
    _u32 => {
      format!("{} bottles of beer on the wall.\n{} bottles of beer. Take one down, pass it around.\n{} bottles of beer on the wall.\n", _u32, _u32, _u32 - 1)
    }
  }
}

fn main() {
  let mut input = String::new();

  println!("Enter the number of bottles: ");

  io::stdin().read_line(&mut input).expect("Unable to read line");

  match input.trim().parse() {
    Ok(bottles) => sing(bottles),
    Err(_why) => {
      println!("Invalid input, enter a number");
    }
  }
}
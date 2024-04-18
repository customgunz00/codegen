use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::{self};

fn randstring() -> String {
  let rng = rand::thread_rng();
  let random_bytes: Vec<u8> = rng.sample_iter(Alphanumeric).take(10).collect();
  let random_string: String = random_bytes.into_iter()
      .map(|byte| char::from(byte))
      .collect();
  return random_string;
}

fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int: Result<i32, _> = input.trim().parse();

    for _ in 0..int.unwrap() {
        println!("{}", randstring());
    }

}
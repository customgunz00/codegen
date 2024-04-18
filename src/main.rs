use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::{self};
use std::process::Command;

fn randstring() -> String {
  let rng = rand::thread_rng();
  let random_bytes: Vec<u8> = rng.sample_iter(Alphanumeric).take(14).collect();
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

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

}
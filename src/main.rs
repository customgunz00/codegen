use rand::distributions::Alphanumeric;
use rand::{random, Rng};

fn randstring() -> String {
  let mut rng = rand::thread_rng();
  let random_bytes: Vec<u8> = rng.sample_iter(Alphanumeric).take(10).collect();
  let random_string: String = random_bytes.into_iter()
      .map(|byte| char::from(byte))
      .collect();
  return random_string;
}

fn main() {
    loop {
        let random_string: String = randstring();
        println!("{}", random_string);
    }
}
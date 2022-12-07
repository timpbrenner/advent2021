use crate::util::open_file;

pub fn start_packet() {
  let contents = open_file("data/day6.txt");

  let count = 14;
  let mut chars = contents.chars();
  let mut signal: Vec<char> = Vec::new();

  for _ in 1..=count {
    signal.push(chars.next().unwrap());
  }

  signal.reverse();

  let mut index = count;
  for c in chars {
    let mut uniqued = signal.clone();
    uniqued.sort_unstable();
    uniqued.dedup();

    if uniqued.len() == count {
      println!("Message {}", index);
      return;
    }

    signal.pop();
    let pop = [c];
    signal.splice(0..0, pop);

    index += 1;
  }
}
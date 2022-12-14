use crate::util::open_file;

// [1,1,3,1,1]
// [1,1,5,1,1]

pub fn pairs() {
  let contents = open_file("data/day13.txt");

  let mut pairs = get_pairs(contents);

  let mut indeces:Vec<u32> = Vec::new();
  for i in 0..pairs.len() {
    let pair = &pairs[i];

    if compare(
      pair[0][1..pair[0].len() - 1].to_string(), 
      pair[1][1..pair[1].len() - 1].to_string() 
    ) {
      indeces.push(i as u32);
    }
  }

  println!("Good Indexes: {:?}", indeces);
}

fn get_pairs(contents:String) -> Vec<Vec<String>> {
  let mut pairs:Vec<Vec<String>> = Vec::new();

  for string_pair in contents.split("\n\n") {
    let mut pair:Vec<String> = Vec::new();

    for part in string_pair.split('\n') {
      pair.push(part.to_string());
    }

    pairs.push(pair);
  }

  return pairs;
}

fn compare(left:String, right:String) -> bool {
  println!("{} - {}", left, right);
  println!("--------------------");

  let mut left_chars = left.chars();
  let mut right_chars = right.chars();

  loop {
    let next_option = left_chars.next();
    if next_option.is_none() {
      break;
    }

    let left_char = next_option.unwrap();
    let right_char = right_chars.next().unwrap();

    if left_char == '[' {
      loop {
        let array_char

      }
    } else {
      let left_int = left_char.to_digit(10).unwrap();

      if right_char == '[' {
        //TO DO
      } else {
        let right_int = right_char.to_digit(10).unwrap();
        println!("COMPARE: {} - {}", left_int, right_int);

        if left_int > right_int {
          return false;
        }
      }
    }

    // For commas
    left_chars.next();
    right_chars.next();
  }

  return true;
}
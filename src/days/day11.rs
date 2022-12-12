use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Monkey {
  operation: fn(u64) -> u64,
  test: fn(u64) -> bool,
  true_destination: u64,
  false_destination: u64,
}

pub fn monkey() {
  let mut monkeys: Vec<Monkey> = Vec::new();
  let mut monkey_items: Vec<VecDeque<u64>> = Vec::from([
    VecDeque::from([84,66,62,69,88,91,91]),
    VecDeque::from([98,50,76,99]),

    VecDeque::from([72,56,94]),
    VecDeque::from([55,88,90,77,60,67]),

    VecDeque::from([69,72,63,60,72,52,63,78]),
    VecDeque::from([89,73]),

    VecDeque::from([78,68,98,88,66]),
    VecDeque::from([70]),
  ]);

  let mut inspection_counts: Vec<u64> = Vec::from([0,0,0,0,0,0,0,0]);

  // 0
  monkeys.push(Monkey{
    operation: |old: u64| old * 11,
    test: |input: u64| input % 2 == 0,
    true_destination: 4,
    false_destination: 7,
  });

  // 1
  monkeys.push(Monkey{
    operation: |old: u64| old * old,
    test: |input: u64| input % 7 == 0,
    true_destination: 3,
    false_destination: 6,
  });

  // 2
  monkeys.push(Monkey{
    operation: |old: u64| old + 1,
    test: |input: u64| input % 13 == 0,
    true_destination: 4,
    false_destination: 0,
  });

  // 3
  monkeys.push(Monkey{
    operation: |old: u64| old + 2,
    test: |input: u64| input % 3 == 0,
    true_destination: 6,
    false_destination: 5,
  });

  // 4
  monkeys.push(Monkey{
    operation: |old: u64| old * 13,
    test: |input: u64| input % 19 == 0,
    true_destination: 1,
    false_destination: 7,
  });

  // 5
  monkeys.push(Monkey{
    operation: |old: u64| old + 5,
    test: |input: u64| input % 17 == 0,
    true_destination: 2,
    false_destination: 0,
  });

  // 6
  monkeys.push(Monkey{
    operation: |old: u64| old + 6,
    test: |input: u64| input % 11 == 0,
    true_destination: 2,
    false_destination: 5,
  });

  // 7
  monkeys.push(Monkey{
    operation: |old: u64| old + 7,
    test: |input: u64| input % 5 == 0,
    true_destination: 1,
    false_destination: 3,
  });

  for round in 1..=10000 {
    for i in 0..monkeys.len() {
      let monkey = &mut monkeys[i];

      while let Some(item) = monkey_items[i].pop_front() {
        let result:u64 = (monkey.operation)(item) % 9699690;
        let new_spot = if (monkey.test)(result) {
          monkey.true_destination
        } else {
          monkey.false_destination
        };

        inspection_counts[i] += 1;
        monkey_items[new_spot as usize].push_back(result);
      }
    }
  }

  inspection_counts.sort();
  inspection_counts.reverse();
  println!("Mokney Business {}", inspection_counts[0] * inspection_counts[1]);
}
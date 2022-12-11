use crate::util::open_file;

#[derive(Debug, Clone)]
struct Command {
  cycles: u32,
  command: String,
}

impl Command {
  fn new(cycles: u32, command: String) -> Command {
    Command { cycles: cycles, command: command }
  }

  fn cycle_and_execute(&mut self, x: i32) -> i32 {
    self.cycles -= 1;

    if self.cycles > 0 {
      return x;
    }

    if self.command == "noop" {
      // println!("Execute Noop");
      return x;
    }

    let mut command_parts = self.command.split(' ').skip(1);
    let integer:i32 = command_parts.next().unwrap().parse::<i32>().unwrap();

    // println!("Execute Addx: {} + {}", x, integer);
    return x + integer;
  }
}

pub fn cycle() {
  let contents = open_file("data/day10_test.txt");

  let mut commands: Vec<Command> = Vec::new();
  let mut x: i32 = 1;
  let mut cycle: u32 = 1;

  for line in contents.lines() {
    println!("{}: {}", cycle, x);
    let mut cycles = 2;
    if line.starts_with("noop") {
      cycles = 1;
    }

    commands.push(Command::new(cycles, line.to_string()));

    for c in &mut commands { 
      x = c.cycle_and_execute(x);
    }

    commands = commands.iter().filter(|c| c.cycles > 0 ).cloned().collect();
    cycle += 1;
  }

  while commands.len() > 0 {
    println!("{}: {}", cycle, x);
    for c in &mut commands { 
      x = c.cycle_and_execute(x);
    }

    commands = commands.iter().filter(|c| c.cycles > 0 ).cloned().collect();
    cycle += 1;
  }
}
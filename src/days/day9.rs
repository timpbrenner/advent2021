use crate::util::open_file;

use std::collections::HashSet;

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn new(x:i32, y:i32) -> Point {
    Point { x: x, y: y }
  }

  fn add(&mut self, x:i32, y:i32) {
    self.x += x;
    self.y += y;
  }

  fn move_to(&mut self, direction: char) {
    match direction {
      'U' => self.add(0,1),
      'D' => self.add(0,-1),
      'R' => self.add(1,0),
      'L' => self.add(-1,0),
      _ => (),
    }
  }

  fn trail(&mut self, lead: Point) {
    if self.x == lead.x && self.y - lead.y >=2 {
      self.y -= 1;
      return;
    }
    if self.x == lead.x && lead.y - self.y >=2 {
      self.y += 1;
      return;
    }

    if self.y == lead.y && self.x - lead.x >=2 {
      self.x -= 1;
      return;
    }
    if self.y == lead.y && lead.x - self.x >=2 {
      self.x += 1;
      return;
    }

    let diagonal_distance = (self.y - lead.y).abs() + (self.x - lead.x).abs();

    if diagonal_distance > 2 {
      let x_diff = if lead.x > self.x { 1 } else { -1 };
      let y_diff = if lead.y > self.y { 1 } else { -1 };

      self.x += x_diff;
      self.y += y_diff;
    }
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && 
        self.y == other.y
  }
}

pub fn move_knot() {
  let contents = open_file("data/day9.txt");

  let mut head = Point::new(0, 0);
  let mut knot_1 = Point::new(0, 0);
  let mut knot_2 = Point::new(0, 0);
  let mut knot_3 = Point::new(0, 0);
  let mut knot_4 = Point::new(0, 0);
  let mut knot_5 = Point::new(0, 0);
  let mut knot_6 = Point::new(0, 0);
  let mut knot_7 = Point::new(0, 0);
  let mut knot_8 = Point::new(0, 0);
  let mut tail = Point::new(0, 0);
  let mut tail_positions:HashSet<Point> = HashSet::new();
  tail_positions.insert(tail);

  for line in contents.lines() {
    let mut line_parts = line.split(' ');
    let direction:char = line_parts.next().unwrap().chars().next().unwrap();
    let count:i32 = line_parts.next().unwrap().parse::<i32>().unwrap();

    for i in 0..count {
      head.move_to(direction);
      knot_1.trail(head);
      knot_2.trail(knot_1);
      knot_3.trail(knot_2);
      knot_4.trail(knot_3);
      knot_5.trail(knot_4);
      knot_6.trail(knot_5);
      knot_7.trail(knot_6);
      knot_8.trail(knot_7);
      tail.trail(knot_8);
      tail_positions.insert(tail);

    }
  }

  println!("Tail Positions: {}", tail_positions.len());
}
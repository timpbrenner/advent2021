use crate::util::open_file;
use regex::Regex;
use std::fmt::*;
use std::collections::HashMap;
use std::io;
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Valve {
  name: String,
  flow: u32,
  branches: Vec<String>,
}

pub fn pipes() {
  let valves = get_valves();

  let mut flow:u32 = 0;
  let mut opened: Vec<String> = Vec::new();
  let mut current:String = "AA".to_string();
  for min in 0..29 {
    println!("");
    println!("TICK({}): {} {}", min + 1, current, flow);
    println!("_____________________");

    flow += get_flow(&valves, &opened);
    tick(&mut current, &valves, &mut opened);
  }

   println!("FLOW: {}", flow);
}

fn get_flow(valves: &HashMap<String, Valve>, opened: &Vec<String>) -> u32 {
  opened.iter().map(|v| valves[v].flow ).sum()
}

fn tick(current:&mut String, valves: &HashMap<String, Valve>, opened: &mut Vec<String>) {
  let current_valve = valves[current].clone();
  if !opened.contains(&current) && current_valve.flow > 0{
    opened.push(current.clone());
    println!("Open Valve: {} - {:?}", current.clone(), opened);
    return;
  }

  let t = get_target(valves, opened);
  if t.is_none() {
    return;
  }
  let target = t.unwrap();
  let mut path = get_path(valves, (*current).clone(), target.clone());
  println!("Target: {} - {:?}", target.clone(), path.clone());

  *current = path.pop_front().unwrap();
}

fn get_path(valves: &HashMap<String, Valve>, current: String, target: String) -> VecDeque<String> {
  let current_valve = &valves[&current];
  let mut path:VecDeque<String> = VecDeque::new();

  for branch in &current_valve.branches {
    // println!("FIND FOR BRANCH: {}", branch);
    let branch_path = rec_path(valves, branch, target.clone(), VecDeque::from([branch.clone()]));

    if (path.len() == 0|| path.len() > branch_path.len()) && branch_path.len() > 0  {
      path = branch_path;
      // println!("Set path {:?}", path);
    }
  }

  return path;
}

fn rec_path(valves: &HashMap<String, Valve>, current: &String, target: String, path: VecDeque<String>) -> VecDeque<String> {
  if current == &target {
    // println!("FOUND: {} - {:?}", path.len(), path);
    return path;
  }

  let current_valve = valves[current].clone();
  let mut r_path:VecDeque<String> = VecDeque::new();
  // println!("Branches: {:?}", &current_valve.branches);
  for branch in &current_valve.branches {
    if path.contains(branch) {
      // println!("SKIP");
      continue;
    }

    let mut branch_path = path.clone();
    branch_path.push_back((&branch).to_string());
    let other_path = rec_path(valves, branch, target.clone(), branch_path);

    if (r_path.len() == 0 || r_path.len() > other_path.len()) && other_path.len() > 0  {
      r_path = other_path;
    }
  }

  return r_path;
}

fn get_target(valves: &HashMap<String, Valve>, opened: &mut Vec<String>) -> Option<String> {
  let result = valves.values().filter(|v| v.flow > 0 && !opened.contains(&v.name)).max_by_key(|x| x.flow);
  if result.is_none() {
    return None;
  }

  Some(result.unwrap().name.clone())
}

fn get_valves() -> HashMap<String, Valve> {
  let contents = open_file("data/day16_test.txt");
  let mut valves = HashMap::new();

  for line in contents.lines() {
    let re = Regex::new(r"Valve (\D+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let cap = re.captures(line).unwrap();

    let name = cap[1].to_string();
    valves.insert(name.clone(), Valve {
      name: name.clone(),
      flow: cap[2].parse::<u32>().unwrap(),
      branches: cap[3].to_string().split(", ").map(|x| x.to_string()).collect::<Vec<String>>()
    });
  }

  return valves;
}
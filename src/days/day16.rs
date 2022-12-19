use crate::util::open_file;
use regex::Regex;
use std::fmt::*;
use std::collections::HashMap;
use std::io;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Valve {
  name: String,
  flow: u32,
  branches: Vec<String>,
  paths: HashMap<String, VecDeque<String>>,
}

pub fn pipes() {
  let mut valves = get_valves();

  let mut flow:u32 = 0;
  let mut opened: Vec<String> = Vec::new();
  let mut current:String = "AA".to_string();
  for min in 0..29 {
    println!("");
    println!("TICK({}): {} {}", min + 1, current, flow);
    println!("_____________________");

    flow += get_flow(&valves, &opened);
    tick(&mut current, &mut valves, &mut opened, min);

   break;
  }

   println!("FLOW: {}", flow);
}

fn get_flow(valves: &HashMap<String, Valve>, opened: &Vec<String>) -> u32 {
  opened.iter().map(|v| valves[v].flow ).sum()
}

fn tick(current:&mut String, valves: &mut HashMap<String, Valve>, opened: &mut Vec<String>, min:u32) {
  let current_valve = valves[current].clone();
  if !opened.contains(&current) && current_valve.flow > 0{
    opened.push(current.clone());
    println!("Open Valve: {} - {:?}", current.clone(), opened);
    return;
  }

  populate_paths(current, valves);
  let t = get_target(current, valves, opened, min);
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

fn get_target(current: &String, valves: &HashMap<String, Valve>, opened: &mut Vec<String>, min:u32) -> Option<String> {
  let current = valves[current].clone();
  println!("PATHS: {:?}", current.paths.values());
  for (name, path) in current.paths {
    let target = valves[&name].clone();
    println!("PATH: {} - {:?}", name, path);
    let time_left = 30 - min - path.len() as u32;
    let expected_amount = time_left * target.flow;

    println!("{} * {} = {}", time_left, target.flow, expected_amount);
  }

  let result = valves.values().filter(|v| v.flow > 0 && !opened.contains(&v.name)).max_by_key(|v|{
    v.flow
  });

  if result.is_none() {
    return None;
  }

  Some(result.unwrap().name.clone())
}

fn populate_paths(current:&String, valves: &mut HashMap<String, Valve>) {
  let pass_in = valves.clone();
  let current_valve = valves.get_mut(current).unwrap();
  for (name, valve) in &pass_in {
    if valve.flow == 0 {
      continue;
    }

    current_valve.paths.insert(name.clone(), get_path(&pass_in, (*current).clone(), name.clone()));
  }
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
      branches: cap[3].to_string().split(", ").map(|x| x.to_string()).collect::<Vec<String>>(),
      paths: HashMap::new()
    });
  }

  return valves;
}
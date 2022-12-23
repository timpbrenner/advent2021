use crate::util::open_file;
use pathfinding::prelude::bfs;
use regex::Regex;
use std::collections::{VecDeque, HashMap};
use std::fmt::*;

#[derive(Clone, Debug)]
struct Valve {
  name: String,
  flow: u32,
  branches: Vec<String>,
  paths: HashMap<String, VecDeque<String>>,
}

pub fn pipes() {
  let mut valves = get_valves();

  for (name, _valve) in valves.clone() {
    populate_paths(&name, &mut valves);
  }

  let current_valve = valves.get("AA").unwrap();
  let (max_flow, steps) = get_max_flow(current_valve, &valves, 1, Vec::new(), 0, Vec::new());

  for step in steps {
    println!("{}", step);
  }

  println!("MAX FLOW: {}", max_flow);
}

fn get_max_flow(current_valve: &Valve, valves: &HashMap<String, Valve>, min: u32, opened: Vec<String>, current_flow: u32, steps: Vec<String>) -> (u32, Vec<String>) {
  let mut flow_flow = current_flow.clone();
  let mut flow_steps = steps.clone();

  if opened.len() == current_valve.paths.len() {
    let opened_flow:u32 = opened.iter().map(|v| valves.get(v).unwrap().flow).sum();
    return (flow_flow + opened_flow * (31 - min), flow_steps);
  }

  if min > 30 {
    return (flow_flow, flow_steps); 
  }

  for (name, path) in &current_valve.paths {
    if opened.contains(name) {
      continue;
    }

    let mut branch_min = min.clone();
    let mut branch_opened = opened.clone();
    let mut branch_flow = current_flow.clone();
    let mut branch_steps = steps.clone();
    let opened_flow:u32 = branch_opened.iter().map(|v| valves.get(v).unwrap().flow).sum();
    let mut branch_valve = current_valve;

    for branch_name in path {
      branch_valve = valves.get(branch_name).unwrap();

      branch_steps.push(format!("== Minute {} ==\nValves are open: {:?}, releasing {} pressure\nYou move to valve {}\n", branch_min, branch_opened, opened_flow, branch_valve.name));
      branch_min += 1;
      branch_flow += opened_flow;

      if branch_min > 30 {
        return (branch_flow, branch_steps); 
      }
    }

    branch_steps.push(format!("== Minute {} ==\nValves are open: {:?}, releasing {} pressure\nYou open valve {}\n", branch_min, branch_opened, opened_flow, branch_valve.name));
    branch_opened.push(branch_valve.name.clone());
    branch_min += 1;
    branch_flow += opened_flow;

    let branch_valve = valves.get(name).unwrap();
    let (max_flow, max_steps) = get_max_flow(branch_valve, valves, branch_min, branch_opened.clone(), branch_flow, branch_steps.clone());
    if max_flow > flow_flow {
      flow_flow = max_flow;
      flow_steps = max_steps;
    }
  }

  return (flow_flow, flow_steps);
}

fn populate_paths(current:&String, valves: &mut HashMap<String, Valve>) {
  let pass_in = valves.clone();
  let current_valve = valves.get_mut(current).unwrap();
  for (name, valve) in &pass_in {
    if valve.flow == 0 {
      continue;
    }

    current_valve.paths.insert(name.clone(), get_algo_path(&pass_in, (*current).clone(), name.clone()));
  }
}

fn get_algo_path(valves: &HashMap<String, Valve>, current: String, target: String) -> VecDeque<String> {
  let result = bfs(&current, |v| valves.clone()[v].branches.clone(), |v| *v == target).unwrap();

  let mut path = VecDeque::from(result);
  path.pop_front();
  path
}

fn get_valves() -> HashMap<String, Valve> {
  let contents = open_file("data/day16.txt");
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

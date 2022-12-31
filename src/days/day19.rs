use crate::util::open_file;
use regex::Regex;
use std::fmt::*;

const FILE:&str = "data/day19.txt";
const MAX_MIN:u32 = 24;

#[derive(Debug, Clone)]
struct Blueprint {
  id: u32,
  ore_robot: u32,
  clay_robot: u32,
  obsidian_robot_ore: u32,
  obsidian_robot_clay: u32,
  geode_robot_ore: u32,
  geode_robot_obsidian: u32,
}

#[derive(Debug, Clone)]
struct GameState {
  min: u32,
  blueprint: Blueprint,

  ore: u32,
  clay: u32,
  obsidian: u32,
  geode: u32,

  ore_robot: u32,
  clay_robot: u32,
  obsidian_robot: u32,
  geode_robot: u32,
}

impl Display for GameState {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Min {}\nOre Robot:{}\nClay Robot:{}\nOre: {}\nClay: {}\nObsidian: {}\nGeode: {}\n", self.min, self.ore_robot, self.clay_robot, self.ore, self.clay, self.obsidian, self.geode)
  }
}

impl GameState {
  fn new(blueprint: Blueprint) -> GameState {
    return GameState {
      min: 1,
      blueprint: blueprint,
      ore: 0,
      clay: 0,
      obsidian: 0,
      geode: 0,
      ore_robot: 1,
      clay_robot: 0,
      obsidian_robot: 0,
      geode_robot: 0,
    };
  }

  fn can_build_geode(&self) -> bool {
    self.ore >= self.blueprint.geode_robot_ore &&
      self.obsidian >= self.blueprint.geode_robot_obsidian
  }

  fn build_geode(&mut self) -> String {
    self.geode_robot += 1;
    self.ore -= self.blueprint.geode_robot_ore;
    self.obsidian -= self.blueprint.geode_robot_obsidian;

    format!("Spend {} ore and {} obsidian to start building an obsidian-collecting robot.", self.blueprint.geode_robot_ore, self.blueprint.geode_robot_obsidian)
  }

  fn can_build_obsidian(&self) -> bool {
    self.ore >= self.blueprint.obsidian_robot_ore &&
      self.clay >= self.blueprint.obsidian_robot_clay
  }

  fn build_obsidian(&mut self) -> String {
    self.obsidian_robot += 1;
    self.ore -= self.blueprint.obsidian_robot_ore;
    self.clay -= self.blueprint.obsidian_robot_clay;
    
    format!("Spend {} ore and {} clay to start building an obsidian-collecting robot.", self.blueprint.obsidian_robot_ore, self.blueprint.obsidian_robot_clay)
  }

  fn can_build_clay(&self) -> bool {
    self.ore >= self.blueprint.clay_robot
  }

  fn build_clay(&mut self) -> String {

    self.clay_robot += 1;
    self.ore -= self.blueprint.clay_robot;

    format!("Spend {} ore to start building a clay-collecting robot.", self.blueprint.clay_robot)
  }

  fn can_build_ore(&self) -> bool {
    self.ore >= self.blueprint.ore_robot
  }

  fn build_ore(&mut self) -> String {
    self.ore_robot += 1;
    self.ore -= self.blueprint.ore_robot;

    format!("Spend {} ore to start building a ore-collecting robot.", self.blueprint.ore_robot)
  }

  fn generate(&self) -> (u32,u32,u32,u32) {
    return (
      self.ore_robot,
      self.clay_robot,
      self.obsidian_robot,
      self.geode_robot,
    )
  }

  fn add_resources(&mut self, g_ore:u32, g_clay:u32, g_obsidian:u32, g_geode:u32) {
    self.ore += g_ore;
    self.clay += g_clay;
    self.obsidian += g_obsidian;
    self.geode += g_geode;
  }
}

pub fn mine() {
  let blueprints = parse_blueprints();
  let mut quality = 0;
  for b in blueprints {
    println!("Blueprint {}", b.id);
    println!("Ore Robot: {}", b.ore_robot);
    println!("Clay Robot: {}", b.clay_robot);
    println!("Obsidian Robot: {} {}", b.obsidian_robot_ore, b.obsidian_robot_clay);
    println!("Geode Robot: {} {}", b.geode_robot_ore, b.geode_robot_obsidian);

    let state = GameState::new(b.clone());
    let (state, steps) = tick(state, Vec::new());

    for s in steps {
      println!("{}",s);
    }

    quality += b.id * state.geode;
    println!("Blueprint({}): {} - {}", b.clone().id, state.geode, b.id * state.geode);

    break;
  }

  println!("Final Quality: {}", quality);
}

fn tick(state: GameState, mut steps: Vec<String>) -> (GameState, Vec<String>) {
  if state.min > MAX_MIN {
    return (state, steps);
  }

  steps.push(format!(""));
  steps.push(format!("== Minute {} ==", state.min));
  let mut start_state = state.clone();
  let (g_ore, g_clay, g_obsidian, g_geode) = state.generate();
  steps.push(format!("{} ore-collecting robot collects {} ore; you now have {} ore.", g_ore, g_ore, g_ore + state.ore));
  if g_clay > 0 {
    steps.push(format!("{} clay-collecting robot collects {} clay; you now have {} clay.", g_clay, g_clay, state.clay + g_clay));
  }

  if g_obsidian > 0 {
    steps.push(format!("{} obsidian-collecting robot collects {} obsidian; you now have {} obsidian.", g_obsidian, g_obsidian, state.obsidian + g_obsidian));
  }

  if g_geode > 0 {
    steps.push(format!("{} geode-collecting robot collects {} geode; you now have {} geode.", g_geode, g_geode, state.geode + g_geode));
  }
  
  if start_state.can_build_geode() {
    steps.push(start_state.build_geode());
  }

  if start_state.can_build_obsidian() {
    steps.push(start_state.build_obsidian());
  }

  let mut clay_final_state: GameState = GameState::new(state.blueprint.clone());
  let mut clay_final_steps: Vec<String> = Vec::new();
  if start_state.can_build_clay() && start_state.clay_robot < 5 {
    let mut clay_state = start_state.clone();
    let mut clay_steps = steps.clone();
    
    clay_steps.push(clay_state.build_clay());
    clay_state.add_resources(g_ore, g_clay, g_obsidian, g_geode);
    clay_state.min += 1;

    (clay_final_state, clay_final_steps) = tick(clay_state, clay_steps);
  }

  let mut ore_final_state: GameState = GameState::new(state.blueprint.clone());
  let mut ore_final_steps: Vec<String> = Vec::new();
  if start_state.can_build_ore() && start_state.clay_robot < 2 && start_state.ore_robot < 5 {
    let mut ore_state = start_state.clone();
    let mut ore_steps = steps.clone();
    
    ore_steps.push(ore_state.build_ore());
    ore_state.add_resources(g_ore, g_clay, g_obsidian, g_geode);
    ore_state.min += 1;

    (ore_final_state, ore_final_steps) = tick(ore_state, ore_steps);
  }

  start_state.add_resources(g_ore, g_clay, g_obsidian, g_geode);
  start_state.min += 1;

  let (final_state, final_steps) = tick(start_state, steps);

  if ore_final_state.geode > final_state.geode && ore_final_state.geode > clay_final_state.geode {
    return (ore_final_state, ore_final_steps);
  }

  if clay_final_state.geode > final_state.geode && clay_final_state.geode > ore_final_state.geode {
    return (clay_final_state, clay_final_steps);
  }

  return (final_state, final_steps);
}

fn parse_blueprints() -> Vec<Blueprint> {
  let mut blueprints:Vec<Blueprint> = Vec::new();

  for line in open_file(FILE).lines() {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let cap = re.captures(line).unwrap();

    blueprints.push(Blueprint{
      id: cap[1].parse::<u32>().unwrap(),
      ore_robot: cap[2].parse::<u32>().unwrap(),
      clay_robot: cap[3].parse::<u32>().unwrap(),
      obsidian_robot_ore: cap[4].parse::<u32>().unwrap(),
      obsidian_robot_clay: cap[5].parse::<u32>().unwrap(),
      geode_robot_ore: cap[6].parse::<u32>().unwrap(),
      geode_robot_obsidian: cap[7].parse::<u32>().unwrap(),
    });
  }

  return blueprints;
}


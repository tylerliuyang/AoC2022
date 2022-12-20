use std::{cmp::max, io::stdin};

type Ore = usize;
type Clay = usize;
type Obsidian = usize;
type Geode = usize;

struct RobotCost {
    ore: Ore,
    clay: Ore,
    obsidian: (Ore, Clay),
    geode: (Ore, Obsidian),
}

enum Robot {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

struct Ores {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
}

impl Ores {
    fn buy(&self, costs: &RobotCost, to_buy: Robot) -> Ores {
        match to_buy {
            Robot::ORE => Ores {
                ore: self.ore - costs.ore,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Robot::CLAY => Ores {
                ore: self.ore - costs.clay,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Robot::OBSIDIAN => Ores {
                ore: self.ore - costs.obsidian.0,
                clay: self.clay - costs.obsidian.1,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Robot::GEODE => Ores {
                ore: self.ore - costs.geode.0,
                clay: self.clay,
                obsidian: self.obsidian - costs.geode.1,
                geode: self.geode,
            },
        }
    }

    fn check(&self, costs: &RobotCost, to_buy: Robot) -> bool {
        match to_buy {
            Robot::ORE => self.ore >= costs.ore,
            Robot::CLAY => self.ore >= costs.clay,
            Robot::OBSIDIAN => self.ore >= costs.obsidian.0 && self.clay >= costs.obsidian.1,
            Robot::GEODE => self.ore >= costs.geode.0 && self.obsidian >= costs.geode.1,
        }
    }

    fn add(&self, amount: (usize, usize, usize, usize)) -> Ores {
        Ores {
            ore: self.ore + amount.0,
            clay: self.clay + amount.1,
            obsidian: self.obsidian + amount.2,
            geode: self.geode + amount.3,
        }
    }
}

fn calculate_blueprint(
    costs: &RobotCost,
    mut ores: Ores,
    mut minute: usize,
    amount: (usize, usize, usize, usize),
) -> usize {
    if minute == 32 {
        return ores.geode;
    }
    let mut tried = (false, false, false, false);
    let mut geodes = [0, 0, 0, 0, 0];
    while minute <= 31 {
        if ores.check(costs, Robot::ORE)
            && !tried.0
            && amount.1 == 0
            && amount.2 == 0
            && amount.3 == 0
        {
            tried.0 = true;
            geodes[0] = calculate_blueprint(
                costs,
                ores.buy(costs, Robot::ORE).add(amount),
                minute + 1,
                (amount.0 + 1, amount.1, amount.2, amount.3),
            );
        }
        if ores.check(costs, Robot::CLAY)
            && !tried.1
            && amount.3 == 0
            && amount.1 / amount.0 <= costs.obsidian.1 / costs.obsidian.0
        {
            tried.1 = true;
            geodes[1] = calculate_blueprint(
                costs,
                ores.buy(costs, Robot::CLAY).add(amount),
                minute + 1,
                (amount.0, amount.1 + 1, amount.2, amount.3),
            );
        }
        if ores.check(costs, Robot::OBSIDIAN) && !tried.2 {
            tried.2 = true;
            geodes[2] = calculate_blueprint(
                costs,
                ores.buy(costs, Robot::OBSIDIAN).add(amount),
                minute + 1,
                (amount.0, amount.1, amount.2 + 1, amount.3),
            );
        }
        if ores.check(costs, Robot::GEODE) && !tried.3 {
            tried.3 = true;
            geodes[3] = calculate_blueprint(
                costs,
                ores.buy(costs, Robot::GEODE).add(amount),
                minute + 1,
                (amount.0, amount.1, amount.2, amount.3 + 1),
            );
        }
        minute += 1;
        ores = ores.add(amount);
    }
    geodes[4] = ores.geode;
    return geodes.iter().fold(0, |acc, item| max(*item, acc));
}

pub fn main() {
    let mut blueprints: Vec<RobotCost> = Vec::new();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(" ").collect::<Vec<&str>>();
        blueprints.push(RobotCost {
            ore: sections.get(6).unwrap().parse().unwrap(),
            clay: sections.get(12).unwrap().parse().unwrap(),
            obsidian: (
                sections.get(18).unwrap().parse().unwrap(),
                sections.get(21).unwrap().parse().unwrap(),
            ),
            geode: (
                sections.get(27).unwrap().parse().unwrap(),
                sections.get(30).unwrap().parse().unwrap(),
            ),
        });

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut quality_level = 1;
    for (i, blueprint) in blueprints.iter().enumerate() {
        quality_level *= calculate_blueprint(
            &blueprint,
            Ores {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            0,
            (1, 0, 0, 0),
        );
        println!("total quality of blueprint {} is: {}", i + 1, quality_level);
    }
    println!("MAX {}", quality_level);
}

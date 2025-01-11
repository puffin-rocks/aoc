use std::cmp::max;
use std::collections::HashMap;
use std::mem;
use itertools::Itertools;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone)]
struct Stats{
    name: String,
    hit_points: isize,
    damage: usize,
    armor:usize
}

#[derive(Debug, Clone)]
struct Item{
    cost: usize,
    damage: usize,
    armor:usize
}

impl Item{
    fn new(cost: usize, damage: usize, armor: usize)-> Self{
        Self{
            cost,
            damage,
            armor
        }
    }
}


pub(crate) struct Advent {
    label: Label,
    enemy: Stats,
    player: Stats,
    weapons: HashMap<String, Item>,
    armors: HashMap<String, Item>,
    rings: HashMap<String, Item>
}
impl Default for Advent {
    fn default() -> Self{
        let mut weapons: HashMap<String, Item> = HashMap::new();
        weapons.insert(String::from("Dagger"), Item::new(8, 4, 0));
        weapons.insert(String::from("Shortsword"), Item::new(10, 5, 0));
        weapons.insert(String::from("Warhammer"), Item::new(25, 6, 0));
        weapons.insert(String::from("Longsword"), Item::new(40, 7, 0));
        weapons.insert(String::from("Greataxe"), Item::new(74, 8, 0));

        let mut armors: HashMap<String, Item> = HashMap::new();
        armors.insert(String::from("Leather"), Item::new(13, 0, 1));
        armors.insert(String::from("Chainmail"), Item::new(31, 0, 2));
        armors.insert(String::from("Splintmail"), Item::new(53, 0, 3));
        armors.insert(String::from("Bandedmail"), Item::new(75, 0, 4));
        armors.insert(String::from("Platemail"), Item::new(102, 0, 5));

        let mut rings: HashMap<String, Item> = HashMap::new();
        rings.insert(String::from("Damage +1"), Item::new(25, 1, 0));
        rings.insert(String::from("Damage +2"), Item::new(50, 2, 0));
        rings.insert(String::from("Damage +3"), Item::new(100, 3, 0));
        rings.insert(String::from("Defense +1"), Item::new(20, 0, 1));
        rings.insert(String::from("Defense +2"), Item::new(40, 0, 2));
        rings.insert(String::from("Defense +3"), Item::new(80, 0, 3));

        Self{
            label: Label::new(21, 2015),
            enemy: Stats{name: String::from("Enemy"), hit_points: 0, damage: 0, armor: 0},
            player: Stats{name: String::from("Player"), hit_points: 100, damage: 0, armor: 0},
            weapons,
            armors,
            rings
        }
    }
}

impl Advent {
    fn solve(&self,
             win: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let a_names = std::iter::once(None)
            .chain(self.armors.keys().map(Some))
            .collect::<Vec<_>>();
        let r1_names = std::iter::once(None)
            .chain(self.rings.keys().map(Some))
            .collect::<Vec<_>>();
        let mut min_cost: Option<usize> = None;
        let mut max_cost: usize = 0;
        for (_, w) in self.weapons.iter(){
            for combo in [a_names.clone(), r1_names.clone(), r1_names.clone()].iter().multi_cartesian_product(){
                let mut player = self.player.clone();
                player.damage += w.damage;
                let mut cost = w.cost;
                let (armor, damage) = match combo.as_slice() {
                    [None, None, None] => (0, 0),
                    [Some(a), None, None] => {
                        let a = self.armors.get(*a).unwrap();
                        cost += a.cost;
                        (a.armor, 0)
                    }
                    [None, Some(r), None] => {
                        let r = self.rings.get(*r).unwrap();
                        cost += r.cost;
                        (r.armor, r.damage)
                    }
                    [None, Some(r1), Some(r2)] if r1 != r2 => {
                        let r1 = self.rings.get(*r1).unwrap();
                        let r2 = self.rings.get(*r2).unwrap();
                        cost += r1.cost + r2.cost;
                        (r1.armor + r2.armor, r1.damage + r2.damage)
                    }
                    [Some(a), Some(r), None] => {
                        let a = self.armors.get(*a).unwrap();
                        let r = self.rings.get(*r).unwrap();
                        cost += a.cost + r.cost;
                        (a.armor + r.armor, r.damage)
                    }
                    [Some(a), Some(r1), Some(r2)] if r1 != r2 => {
                        let a = self.armors.get(*a).unwrap();
                        let r1 = self.rings.get(*r1).unwrap();
                        let r2 = self.rings.get(*r2).unwrap();
                        cost += a.cost + r1.cost + r2.cost;
                        (a.armor + r1.armor + r2.armor, r1.damage + r2.damage)
                    }
                    _ => continue,
                };
                if win && min_cost.map_or(false, |m_cost| cost > m_cost) {
                    continue;
                }
                if !win && cost < max_cost {
                    continue;
                }

                player.armor += armor;
                player.damage += damage;

                if fight(&mut player, &mut self.enemy.clone()) == win {
                    if win {
                        min_cost = Some(min_cost.map_or(cost, |m_cost| m_cost.min(cost)));
                    } else {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
        if win {
            match min_cost {
                None => Err(String::from("No solution found")),
                Some(result) => assert_display(result, None, result_prd, "Min gold spend to win", false)
            }
        }else{
            assert_display(max_cost, None, result_prd, "Max gold spend to lose", false)
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((key, value)) = line.split_once(": "){
            match key{
                "Hit Points" => self.enemy.hit_points = value.parse::<isize>()?,
                "Damage" => self.enemy.damage= value.parse::<usize>()?,
                "Armor" => self.enemy.armor = value.parse::<usize>()?,
                _ => unreachable!()
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Player - Enemy {:?}", (&self.player, &self.enemy));
        Ok(())
    }
    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
       self.solve(true, 78, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 148, 2)
    }
}

fn fight(player: &mut Stats, enemy: &mut Stats) -> bool{
    let attacker = player;
    let defender = enemy;
    while attacker.hit_points>0{
        let damage = max(1, attacker.damage as isize - defender.armor as isize);
        defender.hit_points-=damage;
        mem::swap(defender, attacker);
    }
    attacker.name == "Enemy"
}
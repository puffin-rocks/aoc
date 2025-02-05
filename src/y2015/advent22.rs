use std::cmp::max;
use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone)]
enum Spell{
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Debug, Clone)]
struct Effect{
    time: usize,
    spell: Spell
}

impl Effect{
    fn impact(&mut self, player: &mut Stats, enemy: &mut Stats){
        if self.time>0{
            match self.spell{
                Spell::MagicMissile => enemy.hit_points-=4,
                Spell::Drain => {enemy.hit_points-=2; player.hit_points+=2},
                Spell::Shield => player.armor = 7,
                Spell::Poison => enemy.hit_points-=3,
                Spell::Recharge => player.mana+=101
            }
            self.time-=1;
        }
    }
    fn reset(&mut self){
        if self.time<1{
            match self.spell{
                Spell::MagicMissile | Spell::Drain => self.time = 1,
                Spell::Shield | Spell::Poison => self.time = 6,
                Spell::Recharge => self.time = 5
            }
        }
    }
    fn cost(&self) -> usize{
        match self.spell{
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229
        }
    }
}

#[derive(Debug, Clone)]
struct Stats{
    hit_points: isize,
    damage: usize,
    armor:usize,
    mana: usize
}

pub(crate) struct Advent {
    label: Label,
    enemy: Stats,
    player: Stats,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(22, 2015),
            enemy: Stats{hit_points: 0, damage: 0, armor: 0, mana:0},
            player: Stats{hit_points: 50, damage: 0, armor: 0, mana: 500},
        }
    }
}

impl Advent{
    fn solve(&self,
             hard_mode: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut curr_min_mana: Option<usize> = None;
        let mut memory: HashMap<[usize;6], Option<usize>> = HashMap::new(); //player hp, player mana, boss hp, shield_time, poison_time, recharge_time
        let mut effects: [Effect; 5] = [Effect{time: 0, spell: Spell::MagicMissile},
            Effect{time: 0, spell: Spell::Drain},
            Effect{time: 0, spell: Spell::Shield},
            Effect{time: 0, spell: Spell::Poison},
            Effect{time: 0, spell: Spell::Recharge}
        ];
        let result = player_turn(&mut self.player.clone(), &mut self.enemy.clone(), &mut effects,
                                 &mut memory, 0, &mut curr_min_mana, hard_mode);
        match result{
            Some(mana) => assert_display(mana, None, result_prd, "Least amount of mana", false),
            None => Err(String::from("Player always loses"))
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
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 1824, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, 1937, 2)
    }
}

fn enemy_turn(player: &mut Stats, enemy: &mut Stats, effects: &mut [Effect; 5],
              memory: &mut HashMap<[usize;6], Option<usize>>, total_mana: usize, curr_min_mana: &mut Option<usize>, hard_mode: bool) ->Option<usize>{

    for e in effects.iter_mut(){
        e.impact(player, enemy);
    }
    if enemy.hit_points<1{
        match curr_min_mana{
            None => *curr_min_mana = Some(total_mana),
            Some(m) =>{
                if *m>total_mana{
                   *curr_min_mana = Some(total_mana)
                }
            }
        }
        return Some(total_mana); //won
    }
    let damage = max(1, enemy.damage as isize - player.armor as isize);
    player.hit_points-=damage;
    if player.hit_points<1{
        return None; //lost
    }
    player.armor = 0;
    player_turn(player, enemy, effects, memory, total_mana, curr_min_mana, hard_mode)
}

fn player_turn(player: &mut Stats, enemy: &mut Stats, effects: &mut [Effect; 5],
               memory: &mut HashMap<[usize;6], Option<usize>>, total_mana: usize, curr_min_mana: &mut Option<usize>, hard_mode: bool)->Option<usize>{

    if curr_min_mana.map_or(false, |m| total_mana >= m) {
        return None; // Exceeds current minimum mana
    }

    if hard_mode {
        player.hit_points -= 1;
        if player.hit_points < 1 {
            return None; //lost
        }
    }

    let key = [
        player.hit_points as usize,
        player.mana,
        enemy.hit_points as usize,
        effects[2].time,
        effects[3].time,
        effects[4].time
    ];
    let value = memory.get(&key);
    match value{
        Some(value)=> {
            *value
        },
        None => {
            for e in effects.iter_mut(){
                e.impact(player, enemy);
            }
            player.armor = 0;
            let mut min_mana = None;
            for i in 0..effects.len(){
                let spell_cost = effects[i].cost();
                if effects[i].time<1 && player.mana>=spell_cost{
                    let mut player_next =player.clone();
                    player_next.mana-=spell_cost;
                    let mut effects_next = effects.clone();
                    effects_next[i].reset();
                    if let Some(mana) = enemy_turn(
                        &mut player_next,
                        &mut enemy.clone(),
                        &mut effects_next,
                        memory,
                        total_mana+spell_cost,
                        curr_min_mana,
                        hard_mode){
                            min_mana = Some(min_mana.map_or(mana, |m| if mana < m { mana } else { m }));
                    }

                }
            }
            memory.insert(key, min_mana);
            min_mana
        }
    }
}
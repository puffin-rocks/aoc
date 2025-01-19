use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

#[derive(Clone)]

enum Instruction{
    Direct(usize, usize),
    BotBot(usize, usize, usize),
    OutputBot(usize, usize, usize),
    OutputOutput(usize, usize, usize)
}
impl Instruction{
    fn apply(&self, bots: &mut HashMap<usize, HashSet<usize>>, bins: &mut HashMap<usize, HashSet<usize>>)-> bool{
        match self {
            Instruction::Direct(value,target) =>{
                bots.entry(*target).or_insert_with(HashSet::new).insert(*value);
                return true
            }
            Instruction::BotBot(source, low, high) => {
                if let Some(s) = bots.get(&source){
                    if s.len()==2{
                        let v = s.iter().sorted_by(|&v1,&v2| v1.cmp(v2)).collect::<Vec<_>>();
                        let lower = *v[0];
                        let higher = *v[1];
                        bots.entry(*source).or_insert_with(HashSet::new).clear();
                        bots.entry(*low).or_insert_with(HashSet::new).insert(lower);
                        bots.entry(*high).or_insert_with(HashSet::new).insert(higher);
                        return true
                    }
                }
            }
            Instruction::OutputBot(source, low, high) => {
                if let Some(s) = bots.get(&source){
                    if s.len()==2{
                        let v = s.iter().sorted_by(|&v1,&v2| v1.cmp(v2)).collect::<Vec<_>>();
                        let lower = *v[0];
                        let higher = *v[1];
                        bots.entry(*source).or_insert_with(HashSet::new).clear();
                        bins.entry(*low).or_insert_with(HashSet::new).insert(lower);
                        bots.entry(*high).or_insert_with(HashSet::new).insert(higher);
                        return true
                    }
                }
            }
            Instruction::OutputOutput(source, low, high) => {
                if let Some(s) = bots.get(&source){
                    if s.len()==2{
                        let v = s.iter().sorted_by(|&v1,&v2| v1.cmp(v2)).collect::<Vec<_>>();
                        let lower = *v[0];
                        let higher = *v[1];
                        bots.entry(*source).or_insert_with(HashSet::new).clear();
                        bins.entry(*low).or_insert_with(HashSet::new).insert(lower);
                        bins.entry(*high).or_insert_with(HashSet::new).insert(higher);
                        return true
                    }
                }
            }
        }
        false
    }
}

pub(crate) struct Advent {
    label: Label,
    instructions: Vec<Instruction>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(10, 2016),
            instructions: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some(captures) = Regex::new(r"bot (\d+) gives low to bot (\d+) and high to bot (\d+)").unwrap()
            .captures(&line)
        {
            let source = captures[1].parse::<usize>()?;
            let low = captures[2].parse::<usize>()?;
            let high = captures[3].parse::<usize>()?;
            self.instructions.push(Instruction::BotBot(source, low, high));
        } else if let Some(captures) =
            Regex::new(r"bot (\d+) gives low to output (\d+) and high to bot (\d+)").unwrap().captures(&line)
        {
            let source = captures[1].parse::<usize>()?;
            let low = captures[2].parse::<usize>()?;
            let high = captures[3].parse::<usize>()?;
            self.instructions.push(Instruction::OutputBot(source, low, high));
        } else if let Some(captures) =
            Regex::new(r"value (\d+) goes to bot (\d+)").unwrap().captures(&line)
        {
            let value = captures[1].parse::<usize>()?;
            let target = captures[2].parse::<usize>()?;
            self.instructions.push(Instruction::Direct(value, target));
        } else if let Some(captures) =
            Regex::new(r"bot (\d+) gives low to output (\d+) and high to output (\d+)").unwrap()
                .captures(&line)
        {
            let source = captures[1].parse::<usize>()?;
            let low = captures[2].parse::<usize>()?;
            let high = captures[3].parse::<usize>()?;
            self.instructions.push(Instruction::OutputOutput(source, low, high));
        } else {
            "invalid".parse::<i32>()?;
        };
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of instructions {}", self.instructions.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut bots: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut bins: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut instructions = self.instructions.clone();
        let mut applied: Vec<Instruction> = Vec::new();
        let s = hashset![17usize,61usize];
        let mut bot_number: Option<usize> = None;
        while let Some(instruction) = instructions.pop(){
            if instruction.apply(&mut bots, &mut bins){
                applied.push(instruction);
            }
            else{
                instructions.insert(0, instruction);
            }
            let check = bots.iter().filter_map(|(k,v)| if v==&s{ Some(k)}else{None}).collect::<Vec<_>>();
            if check.len()==1{
                bot_number = Some(*check[0]);
                break;
            }
        }
        match bot_number{
            Some(result)=>assert_display(result, None, 73, "Bot number", false),
            None=>Err(String::from("No solution found"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut bots: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut bins: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut instructions = self.instructions.clone();
        let mut applied: Vec<Instruction> = Vec::new();
        while let Some(instruction) = instructions.pop(){
            if instruction.apply(&mut bots, &mut bins){
                applied.push(instruction);
            }
            else{
                instructions.insert(0, instruction);
            }
        }
        let result =  (0..3usize).map(|i| bins.get(&i).unwrap().iter().next().unwrap()).product::<usize>();
        assert_display(result, None, 3965, "Output 0-1-2 product", false)
    }
}
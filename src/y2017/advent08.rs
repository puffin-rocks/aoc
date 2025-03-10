use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};

struct Operation{
    target_register: String,
    increase: bool,
    increment: isize,
    condition_register: String,
    condition_operator: String,
    condition_value: isize
}

impl Operation{
    fn apply(&self, registers: &mut HashMap<String, isize>){
        let value = *registers.get(&self.condition_register).unwrap_or(&0isize);
        if match self.condition_operator.as_str(){
            "!=" => value!=self.condition_value,
            "==" => value==self.condition_value,
            ">" => value>self.condition_value,
            ">=" => value>=self.condition_value,
            "<=" => value<=self.condition_value,
            "<" => value<self.condition_value,
            _ => {
                unreachable!()
            }
        }{
            let v = registers.entry(self.target_register.clone()).or_insert(0);
            if self.increase{
                *v+=self.increment;
            }else{
                *v-=self.increment;
            }
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    operations: Vec<Operation>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(8, 2017),
            operations: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(parts.len(),7);
        let op = Operation{
            target_register: String::from(parts[0]),
            increase: match parts[1]{
                "inc" => true,
                "dec" => false,
                _ => unreachable!()
            },
            increment: parts[2].parse::<isize>()?,
            condition_register: String::from(parts[4]),
            condition_operator: match parts[5]{
                "!=" | "==" | ">" | ">=" | "<=" | "<" => String::from(parts[5]),
                _ => {
                    unreachable!()
                }
            },
            condition_value: parts[6].parse::<isize>()?
        };
        self.operations.push(op);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of operations: {}", self.operations.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut registers: HashMap<String,isize> = HashMap::new();
        for op in self.operations.iter(){
            op.apply(&mut registers);
        }
        let result = registers.iter().map(|(_,&x)| x).max().unwrap_or(0);
        assert_display(result, None, 7296, "Highest final value", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut result = 0isize;
        let mut registers: HashMap<String,isize> = HashMap::new();
        for op in self.operations.iter(){
            op.apply(&mut registers);
            let current_max = registers.iter().map(|(_,&x)| x).max().unwrap_or(0);
            if current_max>result{
                result = current_max;
            }
        }
        assert_display(result, None, 8186, "Highest interim value", false)
    }
}
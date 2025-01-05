use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};
use regex::Regex;
use crate::hashset;

pub(crate) struct Advent {
    label: Label,
    aunts: HashMap<usize,[Option<usize>;10]>,
    memory: [Option<usize>;10]
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(16, 2015),
            aunts: HashMap::new(),
            memory: [Some(3), Some(7), Some(2), Some(3), Some(0),
                Some(0), Some(5), Some(3), Some(2), Some(1)]
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
        let order = ["children", "cats", "samoyeds", "pomeranians", "akitas",
            "vizslas", "goldfish", "trees", "cars", "perfumes"];
        if let Some(captures) = re.captures(line.as_str()) {
            let nmb = String::from(captures.get(1).unwrap().as_str()).parse::<usize>()?;
            let mut values:[Option<usize>;10] = [None;10];
            for i in 1..=3 {
                let key = captures.get(i*2).unwrap().as_str();
                let value = captures.get(i*2+1).unwrap().as_str().parse::<usize>()?;
                if let Some(index) = order.iter().position(|&x| x == key) {
                    values[index] = Some(value);
                } else {
                    "invalid".parse::<i32>()?;
                }
            }
            self.aunts.insert(nmb,values);
        } else {
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of aunts: {}", self.aunts.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut aunt: Option<usize> = None;
        for (i, a_memory) in self.aunts.iter() {
            if a_memory.iter().zip(self.memory.iter()).all(|(v1, v2)| match v1 {
                Some(_) => v1 == v2,
                None => true,
            }){
                aunt = Some(*i);
                break;
            }
        }
        match aunt{
            Some(nmb) => assert_display(nmb,None,103,"Aunt", false),
            None => Err(String::from("No solution found"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let greater = hashset![1usize, 7usize];
        let fewer= hashset![3usize, 6usize];

        let mut aunt: Option<usize> = None;
        for (i, a_memory) in self.aunts.iter() {
            if a_memory.iter().zip(self.memory.iter()).enumerate().all(|(i, (v1, v2))| match v1 {
                Some(v1) => {
                    let v2 = v2.unwrap();
                    if greater.contains(&i){
                        *v1>v2
                    }else if fewer.contains(&i){
                        *v1<v2
                    }else {
                        *v1 == v2
                    }
                },
                None => true,
            }){
                aunt = Some(*i);
                break;
            }
        }
        match aunt{
            Some(nmb) => assert_display(nmb,None,405,"Aunt", false),
            None => Err(String::from("No solution found"))
        }
    }
}


// In particular, the cats and trees readings indicates that there are greater than that many,
// while the pomeranians and goldfish readings indicate that there are fewer than that many.
use std::collections::{HashMap, HashSet, VecDeque};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    communication: HashMap<usize, HashSet<usize>>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(12, 2017),
            communication: HashMap::new()
        }
    }
}

impl Advent{
    fn get_group_of(&self, seed: usize) -> HashSet<usize>{
        let mut stack:VecDeque<usize> = VecDeque::new();
        stack.push_back(seed);
        let mut group = hashset![seed];
        while let Some(el) = stack.pop_front(){
            for p in self.communication.get(&el).unwrap().iter(){
                if !group.contains(p){
                    stack.push_back(*p);
                }
            }
            group.extend(self.communication.get(&el).unwrap().clone());
        }
        group
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((left, right)) = line.split_once(" <-> "){
            let key = left.parse::<usize>()?;
            let values = right.split(", ")
                .map(|x| x.parse::<usize>().expect("Cannot convert string"))
                .collect::<HashSet<_>>();
            self.communication.insert(key,values);
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of programs: {}", self.communication.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        assert_display(self.get_group_of(0).len(), None,115, "Size of group 0", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut n_groups = 0;
        let mut programs = self.communication.keys().cloned().collect::<HashSet<_>>();
        while let Some(v) = programs.iter().next() {
            programs = programs.difference(&self.get_group_of(*v)).cloned().collect();
            n_groups+=1;
        }
        assert_display(n_groups, None, 221, "Number of groups", false)
    }
}
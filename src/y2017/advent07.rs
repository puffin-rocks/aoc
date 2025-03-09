use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

struct Node{
    weight: usize,
    children: HashSet<String>
}

impl Node{
    fn new(weight: usize, children: HashSet<String>) ->Self{
        Node{
            weight,
            children
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    nodes: HashMap<String, Node>,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(7, 2017),
            nodes: HashMap::new()
        }
    }
}

impl Advent{

    fn get_bottom_node(&self)->Option<String>{
        let mut children: HashSet<String> = HashSet::new();
        for (_, n) in self.nodes.iter(){
            children.extend(n.children.clone());
        }
        self.nodes.keys().cloned().collect::<HashSet<_>>()
            .difference(&children)
            .next().cloned()
    }

    fn compute_tower_weight(&self, bottom_node: &String)->(usize, Option<isize>){
        let n = self.nodes.get(bottom_node).unwrap();
        if n.children.is_empty(){
            (n.weight, None)
        }else{
            let mut weight_check: HashMap<usize,HashSet<String>> = HashMap::new();
            let mut correction: Option<isize> = None;
            for ch in n.children.iter(){
                let (weight, correction_sub) = self.compute_tower_weight(ch);
                if correction_sub.is_some(){
                    correction = correction_sub;
                }
                weight_check.entry(weight).or_insert_with(HashSet::new).insert(ch.clone());
            }
            if weight_check.len()==1{
                (n.weight + (*weight_check.iter().next().unwrap().0)*n.children.len(), correction)
            }else{
                let mut wrong_weight: Option<usize> = None;
                let mut wrong_node: Option<&Node> = None;
                let mut correct_weight: Option<usize> = None;
                for (k, v) in weight_check.iter(){
                    if v.len()==1{
                        wrong_weight = Some(*k);
                        wrong_node = self.nodes.get(v.iter().next().unwrap())
                    }else{
                        correct_weight = Some(*k);
                    }
                }
                let diff = correct_weight.unwrap() as isize - wrong_weight.unwrap() as isize;
                let corrected_weight = wrong_node.unwrap().weight as isize + diff;
                (n.weight + (correct_weight.unwrap())*n.children.len(), Some(corrected_weight))
            }
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"(\w+) \((\d+)\)(?: -> (.*))?").unwrap();

        if let Some(caps) = re.captures(line.as_str()) {
            let name = String::from(caps.get(1).unwrap().as_str());
            let weight: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let children: HashSet<String> = caps.get(3)
                .map(|m| m.as_str().split(", ")
                    .map(|x| String::from(x)).collect())
                .unwrap_or_else(HashSet::new);
            let n = Node::new(weight, children);
            self.nodes.insert(name, n);
        } else {
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of nodes: {}", self.nodes.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        if let Some(bottom_node) = self.get_bottom_node()
        {
            assert_display(bottom_node.as_str(), None, "dtacyn", "Bottom node name", false)
        } else {
            Err(String::from("No solution found"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        if let Some(bottom_node) = self.get_bottom_node()
        {
            let (_, result) = self.compute_tower_weight(&bottom_node);
            if let Some(weight) = result{
                assert_display(weight, None, 521, "Corrected weight", false)
            }else{
                Err(String::from("No solution found"))
            }
        } else {
            Err(String::from("No solution found"))
        }
    }
}
